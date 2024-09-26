# Steel

**Steel is a modular framework for building smart contracts on Solana.** It provides of a set of helper functions, macros, and code patterns for scaffolding smart contracts. Steel is generally designed to be unopinionated, minimizing boilerplate and maximizing flexibility.

## Notes

- This codebase is under active development. All interfaces are subject to change.
- There is currently no CLI, init script, or custom localnet toolchain.
- Use `solana build-sbf` to build your programs.
- The API macros currently do not support IDL generation.
- ~~The account "loaders" currently do not yet return readable or mutable account references.~~

## Getting started

To start building with Steel, simply add it to your workspace dependencies.  

```
cargo add steel
```

We plan to offer a CLI soon to initialize and manage new projects. For now, you're on your own. We recommend forking one of the example programs to get started with the recommended folder structure. To build, use the standard Solana toolchain:

```
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

Steel offers a collection of simple macros for defining your contract API and the basic building blocks of your program. 

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

Steel provides a utility function to streamline the program entrypoint. Securely parse incoming instruction data and dispatch it to handlers.

```rs
mod initialize;

use example_1_api::instruction::MyInstruction;
use initialize::*;
use steel::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction::<MyInstruction>(example_1_api::ID, program_id, data)?;

    match ix {
        MyInstruction::Initialize => process_initialize(accounts, data)?,
        MyInstruction::Add => process_add(accounts, data)?,
    }

    Ok(())
}
```

### Validation

Steel provides a library of composable account validation checks. You can chain these checks together to validate arbitrary account state and parse it into the type you need. 

```rs
use example_1_api::state::Counter;
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
      .has_owner(&example_1_api::ID)?
      .to_account_mut::<Counter>()? 
      .check_mut(|c| c.value <= 42)?;

    // Update state.
    counter.value += 1;

    // Return.
    Ok(())
}
```

### CPIs

Steel offers handful of helper functions for executing common CPIs such as initializing PDAs, creating token accounts, minting tokens, burning tokens, and more. 


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
        signer,
        sender_info,
        receiver_info,
        token_program,
        amount,
    )?;

    // Return.
    Ok(())
}
```
