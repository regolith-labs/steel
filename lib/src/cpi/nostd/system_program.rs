use num_enum::{IntoPrimitive, TryFromPrimitive};
use solana_nostd_entrypoint::{AccountInfoC, InstructionC, NoStdAccountInfo};
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey, system_program};

use super::{cpi_invoke, cpi_invoke_signed};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
pub enum SystemInstruction {
    CreateAccount = 0,
    Assign,
    Transfer,
    CreateAccountWithSeed,
    AdvanceNonceAccount,
    WithdrawNonceAccount,
    InitializeNonceAccount,
    AuthorizeNonceAccount,
    Allocate,
    AllocateWithSeed,
    AssignWithSeed,
    TransferWithSeed,
    UpgradeNonceAccount,
}

#[inline]
fn create_account_ix_data(lamports: &u64, space: &u64, owner: [u8; 32]) -> [u8; 52] {
    let mut instruction_data = [0; 52];
    instruction_data[0] = SystemInstruction::CreateAccount.into();
    instruction_data[4..12].copy_from_slice(&lamports.to_le_bytes());
    instruction_data[12..20].copy_from_slice(&space.to_le_bytes());
    instruction_data[20..52].copy_from_slice(&owner);
    instruction_data
}

#[inline]
pub fn create_account_instruction(
    payer: &NoStdAccountInfo,
    pubkey: &NoStdAccountInfo,
    system_program: &NoStdAccountInfo,
    lamports: &u64,
    space: &u64,
    owner: [u8; 32],
) -> (InstructionC, [AccountInfoC; 3]) {
    let instruction_data = create_account_ix_data(lamports, space, owner);
    let instruction_accounts = [payer.to_meta_c(), pubkey.to_meta_c()];

    let instruction = InstructionC {
        program_id: &system_program::ID,
        accounts: instruction_accounts.as_ptr(),
        accounts_len: instruction_accounts.len() as u64,
        data: instruction_data.as_ptr(),
        data_len: instruction_data.len() as u64,
    };

    let infos = [
        payer.to_info_c(),
        pubkey.to_info_c(),
        system_program.to_info_c(),
    ];
    (instruction, infos)
}

pub fn create_account(
    payer: &NoStdAccountInfo,
    pubkey: &NoStdAccountInfo,
    system_program: &NoStdAccountInfo,
    lamports: &u64,
    space: &u64,
    owner: &Pubkey,
) -> ProgramResult {
    let (instruction, infos) = create_account_instruction(
        payer,
        pubkey,
        system_program,
        lamports,
        space,
        owner.to_bytes(),
    );
    cpi_invoke(instruction, &infos)
}

#[inline]
fn transfer_ix_data(amount: &u64) -> [u8; 12] {
    let mut instruction_data = [0; 12];
    instruction_data[0] = SystemInstruction::Transfer.into();
    instruction_data[4..12].copy_from_slice(&amount.to_le_bytes());
    instruction_data
}

#[inline]
pub fn transfer_instruction(
    from: &NoStdAccountInfo,
    to: &NoStdAccountInfo,
    system_program: &NoStdAccountInfo,
    amount: &u64,
) -> (InstructionC, [AccountInfoC; 3]) {
    let instruction_data = transfer_ix_data(amount);
    let instruction_accounts = [from.to_meta_c(), to.to_meta_c()];

    let instruction = InstructionC {
        program_id: &system_program::ID,
        accounts: instruction_accounts.as_ptr(),
        accounts_len: instruction_accounts.len() as u64,
        data: instruction_data.as_ptr(),
        data_len: instruction_data.len() as u64,
    };

    let infos = [from.to_info_c(), to.to_info_c(), system_program.to_info_c()];
    (instruction, infos)
}

/// When `from` *is not* a PDA.
pub fn transfer(
    from: &NoStdAccountInfo,
    to: &NoStdAccountInfo,
    system_program: &NoStdAccountInfo,
    amount: &u64,
) -> ProgramResult {
    let (instruction, infos) = transfer_instruction(from, to, system_program, amount);
    cpi_invoke(instruction, &infos)
}

/// When `from` *is* a PDA.
pub fn transfer_signed(
    from: &NoStdAccountInfo,
    to: &NoStdAccountInfo,
    system_program: &NoStdAccountInfo,
    amount: &u64,
    pda_seeds: &[&[u8]],
) -> ProgramResult {
    let (instruction, infos) = transfer_instruction(from, to, system_program, amount);
    cpi_invoke_signed(instruction, &infos, pda_seeds)
}

#[inline]
fn allocate_ix_data(space: &u64) -> [u8; 12] {
    let mut instruction_data = [0; 12];
    instruction_data[0] = SystemInstruction::Allocate.into();
    instruction_data[4..12].copy_from_slice(&space.to_le_bytes());
    instruction_data
}

#[inline]
pub fn allocate_instruction(
    target_account: &NoStdAccountInfo,
    system_program: &NoStdAccountInfo,
    space: &u64,
) -> (InstructionC, [AccountInfoC; 2]) {
    let instruction_data = allocate_ix_data(space);
    let instruction_accounts = [target_account.to_meta_c()];

    // Build instruction expected by sol_invoke_signed_c
    let instruction = InstructionC {
        program_id: &system_program::ID,
        accounts: instruction_accounts.as_ptr(),
        accounts_len: instruction_accounts.len() as u64,
        data: instruction_data.as_ptr(),
        data_len: instruction_data.len() as u64,
    };

    // Get infos and seeds
    let infos = [target_account.to_info_c(), system_program.to_info_c()];
    (instruction, infos)
}

pub fn allocate(
    target_account: &NoStdAccountInfo,
    system_program: &NoStdAccountInfo,
    space: &u64,
    pda_seeds: &[&[u8]],
) -> ProgramResult {
    let (instruction, infos) = allocate_instruction(target_account, system_program, space);
    cpi_invoke_signed(instruction, &infos, pda_seeds)
}

#[inline]
fn assign_ix_data(owner: [u8; 32]) -> [u8; 36] {
    let mut instruction_data = [0; 36];
    instruction_data[0] = SystemInstruction::Assign.into();
    instruction_data[4..36].copy_from_slice(&owner);
    instruction_data
}

#[inline]
pub fn assign_instruction(
    pubkey: &NoStdAccountInfo,
    owner: &Pubkey,
    system_program: &NoStdAccountInfo,
) -> (InstructionC, [AccountInfoC; 2]) {
    let instruction_data = assign_ix_data(owner.to_bytes());
    let instruction_accounts = [pubkey.to_meta_c()];

    // Build instruction expected by sol_invoke_signed_c
    let instruction = InstructionC {
        program_id: &system_program::ID,
        accounts: instruction_accounts.as_ptr(),
        accounts_len: instruction_accounts.len() as u64,
        data: instruction_data.as_ptr(),
        data_len: instruction_data.len() as u64,
    };

    // Get infos and seeds
    let infos = [pubkey.to_info_c(), system_program.to_info_c()];
    (instruction, infos)
}

pub fn assign(
    pubkey: &NoStdAccountInfo,
    owner: &Pubkey,
    system_program: &NoStdAccountInfo,
    pda_seeds: &[&[u8]],
) -> ProgramResult {
    let (instruction, infos) = assign_instruction(pubkey, owner, system_program);
    cpi_invoke_signed(instruction, &infos, pda_seeds)
}
