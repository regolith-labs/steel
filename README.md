# 🏗️ Steel 

**Steel is a new Solana smart contract framework.** It provides a library of helper functions, macros, and code patterns for building safe and maintainable smart contracts on the Solana blockchain.

## Notes

- **Steel is under active development. All interfaces are subject to change.**
- **This code is unaudited. Use at your own risk!**

## Todos

- [ ] Localnet toolchain.
- [ ] Mainnet toolchain.
- [ ] Passthrough cargo args.
- [ ] IDL generation.
- [x] ~~Helper functions for simple lamport transfers.~~
- [x] ~~Helper functions to emit events (wrap sol_log_data).~~
- [x] ~~Custom error messages on account validation checks.~~
- [x] ~~Helper function to close AccountInfos.~~
- [x] ~~CLI with init script.~~
- [x] ~~Account parsers and validation.~~

## Getting started

To get started, install the CLI:
```sh
cargo install steel-cli
```

Use the `new` command to create a new project:
```sh
steel new my-project
```

Compile your program using the Solana toolchain:
```sh
steel build
```

Test your program using the Solana toolchain:
```sh
steel test
```

## File structure

While not strictly enforced, we recommend organizing your Solana program with the following file structure. We have found this pattern to improve code readability, separating the contract interface from its implementation. It scales well for complex contracts. 

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

### Accounts

Use the `account!` macro to link account structs with a discriminator and implement basic serialization logic.

```rs
use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum MyAccount {
    Counter = 0,
    Profile = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Counter {
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Profile {
    pub id: u64,
}

account!(MyAccount, Counter);
account!(MyAccount, Profile);
```

### Instructions

Use the `instruction!` macro to link instruction data with a discriminator and implement basic serialization logic.

```rs
use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum MyInstruction {
    Add = 0,
    Initialize = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Add {
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {}

instruction!(MyInstruction, Add);
instruction!(MyInstruction, Initialize);

```

### Errors

Use the `error!` macro to define custom errors.

```rs
use steel::*;

#[repr(u32)]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
pub enum MyError {
    #[error("You did something wrong")]
    Dummy = 0,
}

error!(MyError);
```

### Events

Use the `event!` macro to define custom events.

```rs
use steel::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct MyEvent {
    pub value: u64,
}

event!(MyEvent);
```

## Program

### Entrypoint

Use the `entrypoint!` macro to streamline the program entrypoint.

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
        MyInstruction::Add => process_add(accounts, data)?,
        MyInstruction::Initialize => process_initialize(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
```

### Validation

Use chainable parsers and assertions to validate account data.

```rs
use example_api::prelude::*;
use steel::*;

pub fn process_add(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    let [signer_info, counter_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;

    let counter = counter_info
        .as_account_mut::<Counter>(&example_api::ID)? 
        .assert_mut(|c| c.value <= 42)?;

    counter.value += 1;

    Ok(())
}
```

### CPIs

Use helper functions to execute common tasks like creating accounts and transferring tokens.

```rs
use steel::*;

pub fn process_transfer(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    let [signer_info, counter_info, mint_info, sender_info, receiver_info, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;

    counter_info
        .as_account::<Counter>(&example_api::ID)?
        .assert(|c| c.value >= 42)?;

    mint_info.as_mint()?;

    sender_info
        .is_writable()?
        .as_token_account()?
        .assert(|t| t.owner == *signer_info.key)?
        .assert(|t| t.mint == *mint_info.key)?;

    receiver_info
        .is_writable()?
        .as_token_account()?
        .assert(|t| t.mint == *mint_info.key)?;

    token_program.is_program(&spl_token::ID)?;

    transfer(
        signer_info,
        sender_info,
        receiver_info,
        token_program,
        counter.value,
    )?;

    Ok(())
}
```
