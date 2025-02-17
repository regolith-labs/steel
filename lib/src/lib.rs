mod cpi;
mod loaders;
mod log;
pub mod macros;
mod traits;
mod utils;

pub use cpi::*;
pub use log::*;
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
pub use pinocchio::{account_info::AccountInfo, entrypoint, msg, pubkey::Pubkey, ProgramResult};
