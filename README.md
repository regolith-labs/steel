# 🏗️ Steel 

**Steel is a new framework for building smart contracts on Solana.** It provides a library of helper functions, macros, and code patterns for implementing secure and maintainable smart contracts. Steel is generally designed to be unopinionated, minimizing boilerplate code and maximizing developer flexibility.

## Notes

- **Steel is under active development. All interfaces are subject to change.**
- **This code is unaudited. Use at your own risk**

## Todos

- [ ] Localnet toolchain.
- [ ] IDL generation.
- [ ] ~~Helper functions for simple lamport transfers between AccountInfos.~~
- [x] ~~Helper functions to emit events (wrap sol_log_data).~~
- [x] ~~Custom error messages on account validation checks.~~
- [x] ~~Helper function to close AccountInfos (wrap realloc and lamport return).~~
- [x] ~~CLI with init script.~~
- [x] ~~Account parsers and validation.~~

## Getting started

To start building with Steel, install the CLI:
```sh
cargo install steel-cli
```

Spin up a new project with `new` command:
```sh
steel new my-project
```

To compile your program, use the standard Solana toolchain:
```sh
cargo build-sbf
```

## Folder structure

While not strictly enforced, we recommend organizing your Solana program with the following file structure. We have found this pattern improves code readability, separating the contract interface from its implementation, and scales well as contract complexity increases. 

```
Cargo.toml (workspace)
⌙ api
  ⌙ Cargo.toml
  ⌙ src
    ⌙ consts.rs
    ⌙ error.rs
    ⌙ event.rs
    ⌙ instruction.rs
    ⌙ lib.rs
    ⌙ loaders.rs
    ⌙ sdk.rs
    ⌙ state
      ⌙ mod.rs
      ⌙ account_1.rs
      ⌙ account_2.rs
⌙ program
  ⌙ Cargo.toml
  ⌙ src
    ⌙ lib.rs
    ⌙ instruction_1.rs
    ⌙ instruction_2.rs
```

## API

Steel offers a collection of simple macros for defining the interface and building blocks of your program. 

### Accounts

For accounts, Steel uses a single enum to manage discriminators and a struct for each account type. The `account!` macro helps link these types and implements basic serialization logic.

```rs
use steel::*;

/// Enum for account discriminators.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum MyAccount {
    Counter = 0,
}

/// Struct for account state.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Counter {
    pub value: u64,
}

account!(MyAccount, Counter);
```

### Instructions

For instructions, Steel similarly uses a single enum to manage discriminators and a struct for each instruction args type. The `instruction!` macro helps link these types and implement basic serialization logic.

```rs
use steel::*;

/// Enum for instruction discriminators.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum MyInstruction {
    Initialize = 0,
    Add = 1,
}

/// Struct for instruction args.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {}

/// Struct for instruction args.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Add {
    pub value: u64,
}

instruction!(MyInstruction, Initialize);
instruction!(MyInstruction, Add);

```

### Errors

Custom program errors can be created simply by defining an enum for your error messages and passing it to the `error!` macro. 

```rs
use steel::*;

/// Enum for error types.
#[repr(u32)]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
pub enum MyError {
    #[error("You did something wrong")]
    Dummy = 0,
}

error!(MyError);
```

### Events

Similarly, custom program events can be created by defining the event struct and passing it to the `event!` macro. 

```rs
use steel::*;

/// Struct for logged events.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct MyEvent {
    pub value: u64,
}

event!(MyEvent);
```

## Program

In your contract implementation, Steel offers a series of composable functions to parse accounts, validate state, and execute CPIs. 

### Entrypoint

Steel provides a utility function to streamline the program entrypoint. Securely parse incoming instruction data and dispatch it to a handler.

```rs
mod add;
mod initialize;

use add::*;
use initialize::*;

use example_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction::<MyInstruction>(&example_api::ID, program_id, data)?;

    match ix {
        MyInstruction::Initialize => process_initialize(accounts, data)?,
        MyInstruction::Add => process_add(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
```

### Validation

Steel provides a library of composable functions for validating account data. You can chain these functions together to validate arbitrary account state and parse it into whatever type you need. 

```rs
use example_api::prelude::*;
use steel::*;

pub fn process_add(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, counter_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate signer.
    signer_info.is_signer()?;

    // Parse and validate account.
    let counter = counter_info
        .to_account_mut::<Counter>(&example_api::ID)? 
        .check_mut(|c| c.value <= 42)?;

    // Update state.
    counter.value += 1;

    // Return.
    Ok(())
}
```

### CPIs

Steel offers a handful of helper functions for executing common CPIs such as creating accounts, creating token accounts, minting tokens, burning tokens, and more. 


```rs
use steel::*;

pub fn process_transfer(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, mint_info, sender_info, receiver_info, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;

    mint_info.to_mint()?;

    sender_info
        .is_writable()?
        .to_token_account()?
        .check(|t| t.owner == *signer_info.key)?
        .check(|t| t.mint == *mint_info.key)?;

    receiver_info
        .is_writable()?
        .to_token_account()?
        .check(|t| t.mint == *mint_info.key)?;

    token_program.is_program(&spl_token::ID)?;

    // Transfer tokens.
    let amount = 42;
    transfer(
        signer_info,
        sender_info,
        receiver_info,
        token_program,
        amount,
    )?;

    // Return.
    Ok(())
}
```
