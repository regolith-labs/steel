#[cfg(not(feature = "pinocchio"))]
mod cpi;
mod loaders;
mod log;
pub mod macros;
#[cfg(feature = "pinocchio")]
mod pinocchio_cpi;
mod traits;
mod utils;

#[cfg(not(feature = "pinocchio"))]
pub use cpi::*;
pub use log::*;
#[cfg(feature = "pinocchio")]
pub use pinocchio_cpi::*;
pub use traits::*;
pub use utils::*;

pub use bytemuck::{Pod, Zeroable};
pub use num_enum::{IntoPrimitive, TryFromPrimitive};

#[cfg(not(feature = "pinocchio"))]
pub use solana_program::{
    account_info::AccountInfo,
    clock::Clock,
    declare_id, entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program, sysvar,
    sysvar::Sysvar,
};

pub use thiserror::Error;

#[cfg(feature = "pinocchio")]
pub use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{AccountMeta, Instruction},
    msg,
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    ProgramResult,
};

#[cfg(feature = "pinocchio")]
pub use pinocchio_pubkey::declare_id;

#[cfg(feature = "pinocchio")]
pub use pinocchio_system as system_program;
