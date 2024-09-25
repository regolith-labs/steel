# Steel

**Steel is a modular framework for Solana smart contract development.** It provides of a set of helper functions, macros, and code patterns for scaffolding smart contracts. Steel is generally designed to be unopinionated, minimizing boilerplate and maximizing flexibility.

## Notes

- This codebase is under active development. All interfaces are subject to change.
- There is currently no CLI, init script, or custom localnet toolchain.
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

account!(MyAccount, Bus);
```

### Instructions

```rs
use steel::*;

/// Enum for instruction discriminators.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum MyInstruction {
    Update = 0,
}

/// Struct for instruction args.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Increment {
    pub value: u64,
}

instruction!(MyInstruction, Increment);
```

### Errors

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

In your instruction implementations, Steel offers helper functions for validating common types of accounts and executing CPIs. 

### Entrypoint

```rs
mod initialize;

use example_0_api::instruction::MyInstruction;
use initialize::*;
use steel::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction::<MyInstruction>(example_0_api::id(), program_id, data)?;

    match ix {
        MyInstruction::Initialize => process_initialize(accounts, data)?,
    }

    Ok(())
}
```

### Loaders

```rs
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;

    // Return.
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

    // Return
    Ok(())
}
```
