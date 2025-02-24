use bytemuck::Pod;
use pinocchio::{
    account_info::AccountInfo,
    instruction::Instruction,
    pubkey::{find_program_address, Pubkey},
    signer,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use crate::Discriminator;

/// Creates a new account.
#[inline(always)]
pub fn create_account(
    from: &AccountInfo,
    to: &AccountInfo,
    space: u64,
    owner: &[u8; 32],
) -> ProgramResult {
    let lamports = (Rent::get()?).minimum_balance(space as usize);

    pinocchio_system::instructions::CreateAccount {
        from,
        to,
        lamports,
        space,
        owner,
    }
    .invoke()?;

    Ok(())
}

/// Creates a new program account.
#[inline(always)]
pub fn create_program_account<const SIZE: usize, T: Discriminator + Pod>(
    target_account: &AccountInfo,
    payer: &AccountInfo,
    owner: &[u8; 32],
    seeds: &[u8; SIZE],
) -> ProgramResult {
    let chunks: &[&[u8]] = &split_bytes(&seeds, SIZE);

    create_program_account_with_bump::<SIZE, T>(
        target_account,
        payer,
        owner,
        seeds,
        find_program_address(chunks, owner).1,
    )
}

/// Creates a new program account with user-provided bump.
#[inline(always)]
pub fn create_program_account_with_bump<const SIZE: usize, T: Discriminator + Pod>(
    target_account: &AccountInfo,
    payer: &AccountInfo,
    owner: &[u8; 32],
    seeds: &[u8; SIZE],
    bump: u8,
) -> ProgramResult {
    // Allocate space.
    allocate_account_with_bump(
        target_account,
        payer,
        8 + std::mem::size_of::<T>(),
        owner,
        seeds,
        bump,
    )?;

    // Set discriminator.
    let mut data = target_account.try_borrow_mut_data()?;
    data[0] = T::discriminator();

    Ok(())
}

/// Allocates space for a new program account.
#[inline(always)]
pub fn allocate_account<const SIZE: usize>(
    target_account: &AccountInfo,
    payer: &AccountInfo,
    space: usize,
    owner: &[u8; 32],
    seeds: &[u8; SIZE],
) -> ProgramResult {
    let chunks: &[&[u8]] = &split_bytes(&seeds, SIZE);

    allocate_account_with_bump(
        target_account,
        payer,
        space,
        owner,
        seeds,
        find_program_address(chunks, owner).1,
    )
}

/// Allocates space for a new program account with user-provided bump.
#[inline(always)]
pub fn allocate_account_with_bump<const SIZE: usize>(
    target_account: &AccountInfo,
    payer: &AccountInfo,
    space: usize,
    owner: &[u8; 32],
    seeds: &[u8; SIZE],
    bump: u8,
) -> ProgramResult {
    // Combine seeds
    let bump: &[u8] = &[bump];

    // Allocate space for account
    let rent = Rent::get()?;
    if target_account.lamports().eq(&0) {
        // If balance is zero, create account
        pinocchio_system::instructions::CreateAccount {
            from: payer,
            to: target_account,
            lamports: rent.minimum_balance(space),
            space: space as u64,
            owner,
        }
        .invoke_signed(&[signer!(seeds, bump)])?;
    } else {
        // Otherwise, if balance is nonzero:

        // 1) transfer sufficient lamports for rent exemption
        let rent_exempt_balance = rent
            .minimum_balance(space)
            .saturating_sub(target_account.lamports());
        if rent_exempt_balance.gt(&0) {
            pinocchio_system::instructions::Transfer {
                from: payer,
                to: target_account,
                lamports: rent_exempt_balance,
            }
            .invoke()?;
        }

        // 2) allocate space for the account
        pinocchio_system::instructions::Allocate {
            account: target_account,
            space: space as u64,
        }
        .invoke()?;

        // 3) assign our program as the owner
        pinocchio_system::instructions::Assign {
            account: target_account,
            owner,
        }
        .invoke_signed(&[signer!(seeds, bump)])?;
    }

    Ok(())
}

/// Closes an account and returns the remaining rent lamports to the provided recipient.
#[inline(always)]
pub fn close_account(account_info: &AccountInfo, recipient: &AccountInfo) -> ProgramResult {
    // Realloc data to zero.
    account_info.realloc(0, true)?;

    // Return rent lamports.
    *recipient
        .try_borrow_mut_lamports()
        .expect("Failed to borrow mut lamports") += account_info.lamports();
    *account_info
        .try_borrow_mut_lamports()
        .expect("Failed to borrow mut lamports") = 0;

    Ok(())
}

/// Invokes a CPI with provided signer seeds and program id.
#[inline(always)]
pub fn invoke_signed<const ACCOUNTS: usize, const SIZE: usize>(
    instruction: &Instruction,
    account_infos: &[&AccountInfo; ACCOUNTS],
    program_id: &Pubkey,
    seeds: &[u8; SIZE],
) -> ProgramResult {
    let chunks: &[&[u8]] = &split_bytes(&seeds, SIZE);
    let bump = find_program_address(chunks, program_id).1;

    invoke_signed_with_bump(instruction, account_infos, seeds, bump)
}
fn split_bytes<const N: usize>(data: &[u8; N], chunk_size: usize) -> Vec<&[u8]> {
    data.chunks(chunk_size).collect()
}

/// Invokes a CPI with the provided signer seeds and bump.
#[inline(always)]
pub fn invoke_signed_with_bump<const ACCOUNTS: usize, const SIZE: usize>(
    instruction: &Instruction,
    account_infos: &[&AccountInfo; ACCOUNTS],
    seeds: &[u8; SIZE],
    bump: u8,
) -> ProgramResult {
    // Combine seeds
    let bump: &[u8] = &[bump];

    // Invoke CPI
    pinocchio::program::invoke_signed(instruction, account_infos, &[signer!(seeds, bump)])
}
