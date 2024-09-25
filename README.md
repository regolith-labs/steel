# Steel

**Steel is a minimal framework for Solana smart contract development.** It provides of a set of helper functions, macros, and code patterns for organizing contract codebases. It is generally designed to be an unopinionated toolkit, reducing boilerplate and maximizing flexibility.

## Notes

- This codebase is under active development. All interfaces are subject to change.
- There is currently no CLI, scaffolding script, or custom localnet toolchain.
- Use `solana build-sbf` to build your programs.
- The account "loaders" currently do not yet return readable or mutable account references.
- The API macros currently do not support IDL generation.

## File structure

While not strictly enforced, we recommend organizing your Solana program with the following file structure. We have found this pattern improves code readability, separating the contract interface from its implementation, and scales well for more complex contracts. 

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

```rs
use bytemuck::{Pod, Zeroable};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum MyAccount {
    Counter = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Counter {
    pub value: u64,
}

account!(MyAccount, Bus);
```

### Instructions

```rs
use bytemuck::{Pod, Zeroable};
use num_enum::TryFromPrimitive;
use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum MyInstruction {
    Update = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Increment {
    pub value: u64,
}

instruction!(MyInstruction, Increment);
```

### Errors

```rs
use num_enum::IntoPrimitive;
use steel::*;
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum MyError {
    #[error("You did something wrong")]
    Dummy = 0,
}

error!(MyError);
```

### Events

```rs
use bytemuck::{Pod, Zeroable};
use steel::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct MyEvent {
    pub value: u64,
}

event!(MyEvent);
```

## Program

In your instruction implementations, Steel offers helper functions for validating common types of accounts and executing CPIs. 

### Loaders

```rs
use steel::*;

/// Initialize ...
pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;

    // Return ok
    Ok(())
}
```

### CPIs

```rs
use steel::*;

pub fn process_transfer(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer, mint_info, sender_info, receiver_info, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_any_mint(mint_info, false)?;
    load_token_account(sender_info, Some(signer.key), mint_info.key, true)?;
    load_token_account(receiver_info, None, mint_info.key, true)?;
    load_program(token_program, spl_token::id())?;

    // Transfer tokens from sender to receiver.
    let amount = 42;
    transfer(
        signer,
        sender_info,
        receiver_info,
        token_program,
        amount,
    )?;

    Ok(())
}
```
