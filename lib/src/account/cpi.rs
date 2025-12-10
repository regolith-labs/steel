#![allow(deprecated)]
use bytemuck::Pod;

use pinocchio::sysvars::rent::{
    Rent, DEFAULT_BURN_PERCENT, DEFAULT_EXEMPTION_THRESHOLD, DEFAULT_LAMPORTS_PER_BYTE_YEAR,
};
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Instruction, Seed, Signer},
    pubkey::Pubkey,
    ProgramResult,
};

// TODO: remove with bump funcs?

// use crate::{CloseAccount};
use crate::Discriminator;

/// Invokes a CPI with provided signer seeds and program id.
#[inline(always)]
pub fn invoke_signed<const N: usize, const ACCOUNTS: usize>(
    instruction: &Instruction,
    account_infos: &[&AccountInfo; ACCOUNTS],
    seeds: &[Seed; N],
) -> ProgramResult {
    invoke_signed_with_bump(instruction, account_infos, seeds)
}

/// Invokes a CPI with the provided signer seeds and bump.
#[inline(always)]
pub fn invoke_signed_with_bump<const N: usize, const ACCOUNTS: usize>(
    instruction: &Instruction,
    account_infos: &[&AccountInfo; ACCOUNTS],
    seeds: &[Seed; N],
) -> ProgramResult {
    // Combine seeds
    let signer_seeds = Signer::from(seeds);

    // Invoke CPI
    pinocchio::cpi::invoke_signed(instruction, account_infos, &[signer_seeds])
}

/// Creates a new account.
#[inline(always)]
pub fn create_account<'a, 'info>(
    from: &'a AccountInfo,
    to: &'a AccountInfo,
    space: usize,
    owner: &Pubkey,
) -> ProgramResult {
    let rent = Rent {
        lamports_per_byte_year: DEFAULT_LAMPORTS_PER_BYTE_YEAR,
        exemption_threshold: DEFAULT_EXEMPTION_THRESHOLD,
        burn_percent: DEFAULT_BURN_PERCENT,
    };

    let lamports_required = rent.minimum_balance(space);

    pinocchio_system::instructions::CreateAccount {
        from,
        to,
        lamports: lamports_required,
        space: space as u64,
        owner,
    }
    .invoke()?;

    Ok(())
}

/// Creates a new program account.
#[inline(always)]
pub fn create_program_account<'a, 'info, T: Discriminator + Pod, const N: usize>(
    target_account: &'a AccountInfo,
    payer: &'a AccountInfo,
    owner: &Pubkey,
    seeds: &[Seed; N],
) -> ProgramResult {
    create_program_account_with_bump::<T, N>(target_account, payer, owner, seeds)
}

/// Creates a new program account with user-provided bump.
#[inline(always)]
pub fn create_program_account_with_bump<'a, 'info, T: Discriminator + Pod, const N: usize>(
    target_account: &'a AccountInfo,
    payer: &'a AccountInfo,
    owner: &Pubkey,
    seeds: &[Seed; N],
) -> ProgramResult {
    // Allocate space.
    allocate_account_with_bump(
        target_account,
        payer,
        8 + std::mem::size_of::<T>(),
        owner,
        seeds,
    )?;

    // Set discriminator.
    let mut data = target_account.try_borrow_mut_data()?;
    data.copy_from_slice(&T::discriminator().to_le_bytes());

    Ok(())
}

/// Allocates space for a new program account.
#[inline(always)]
pub fn allocate_account<'a, 'info, const N: usize>(
    target_account: &'a AccountInfo,
    payer: &'a AccountInfo,
    space: usize,
    owner: &Pubkey,
    seeds: &[Seed; N],
) -> ProgramResult {
    allocate_account_with_bump(target_account, payer, space, owner, seeds)
}

/// Allocates space for a new program account with user-provided bump.
#[inline(always)]
pub fn allocate_account_with_bump<'a, 'info, const N: usize>(
    target_account: &'a AccountInfo,
    payer: &'a AccountInfo,
    space: usize,
    owner: &Pubkey,
    seeds: &[Seed; N],
) -> ProgramResult {
    let signer_seeds = Signer::from(seeds);

    // Allocate space for account
    let rent = Rent {
        lamports_per_byte_year: DEFAULT_LAMPORTS_PER_BYTE_YEAR,
        exemption_threshold: DEFAULT_EXEMPTION_THRESHOLD,
        burn_percent: DEFAULT_BURN_PERCENT,
    };
    if target_account.lamports().eq(&0) {
        // If balance is zero, create account

        pinocchio_system::instructions::CreateAccount {
            from: payer,
            to: target_account,
            lamports: rent.minimum_balance(space),
            space: space as u64,
            owner,
        }
        .invoke_signed(&[signer_seeds.clone()])?;
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
        .invoke_signed(&[signer_seeds.clone()])?;

        // 3) assign our program as the owner
        pinocchio_system::instructions::Assign {
            account: target_account,
            owner,
        }
        .invoke_signed(&[signer_seeds])?;
    }

    Ok(())
}

/// Closes an account and returns the remaining rent lamports to the provided recipient.
#[inline(always)]
pub fn close_account(account_info: &AccountInfo, recipient: &AccountInfo) -> ProgramResult {
    let lamports = account_info.lamports();

    *account_info.try_borrow_mut_lamports()? -= lamports;
    *recipient.try_borrow_mut_lamports()? += lamports;

    account_info.close()
}
