mod account;
mod log;
pub mod macros;
mod numeric;
#[cfg(feature = "spl")]
mod spl;
mod utils;

pub use account::*;
pub use log::*;
pub use numeric::*;
#[cfg(feature = "spl")]
pub use spl::*;
pub use utils::*;

pub use bytemuck::{Pod, Zeroable};
pub use num_enum::{IntoPrimitive, TryFromPrimitive};
pub use solana_account_view::AccountView;
pub use solana_address::{address, Address};
pub use solana_instruction_view::InstructionView;
pub use solana_program_error::{ProgramError, ProgramResult};

// use solana_clock::Clock;
// use solana_declare_id::declare_id;
// use solana_entrypoint::entrypoint;
// use solana_entrypoint::ProgramResult;
// use solana_instruction::Instruction;

// pub use solana_program::{
//     account_info::AccountInfo,
//     clock::Clock,
//     declare_id, entrypoint,
//     entrypoint::ProgramResult,
//     instruction::{AccountMeta, Instruction},
//     program_error::ProgramError,
//     pubkey::Pubkey,
//     system_program, sysvar,
//     sysvar::Sysvar,
// };
pub use thiserror::Error;
