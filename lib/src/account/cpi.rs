use bytemuck::Pod;

use pinocchio::cpi;
use pinocchio::sysvars::rent::Rent;
use pinocchio::sysvars::Sysvar;
use pinocchio::{
    cpi::{Seed, Signer},
    instruction::InstructionView,
    AccountView, Address, ProgramResult,
};

// use crate::{CloseAccount};
use crate::Discriminator;

/// Invokes a CPI with provided signer seeds and program id.
#[inline(always)]
pub fn invoke_signed<const N: usize, const ACCOUNTS: usize>(
    instruction: &InstructionView,
    account_views: &[&AccountView; ACCOUNTS],
    seeds: &[Seed; N],
) -> ProgramResult {
    let signers_seeds = Signer::from(seeds);

    cpi::invoke_signed(instruction, account_views, &[signers_seeds])
}

/// Creates a new account.
#[inline(always)]
pub fn create_account<'a, 'info>(
    from: &'a AccountView,
    to: &'a AccountView,
    space: usize,
    owner: &Address,
) -> ProgramResult {
    let rent = Rent::get()?;

    let lamports_required = rent.try_minimum_balance(space)?;

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
    target_account: &'a AccountView,
    payer: &'a AccountView,
    owner: &Address,
    seeds: &[Seed; N],
) -> ProgramResult {
    create_program_account_with_bump::<T, N>(target_account, payer, owner, seeds)
}

/// Creates a new program account with user-provided bump.
#[inline(always)]
pub fn create_program_account_with_bump<'a, 'info, T: Discriminator + Pod, const N: usize>(
    target_account: &'a AccountView,
    payer: &'a AccountView,
    owner: &Address,
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
    let mut data = target_account.try_borrow_mut()?;
    data.copy_from_slice(&T::discriminator().to_le_bytes());

    Ok(())
}

/// Allocates space for a new program account.
#[inline(always)]
pub fn allocate_account<'a, 'info, const N: usize>(
    target_account: &'a AccountView,
    payer: &'a AccountView,
    space: usize,
    owner: &Address,
    seeds: &[Seed; N],
) -> ProgramResult {
    allocate_account_with_bump(target_account, payer, space, owner, seeds)
}

/// Allocates space for a new program account with user-provided bump.
#[inline(always)]
pub fn allocate_account_with_bump<'a, 'info, const N: usize>(
    target_account: &'a AccountView,
    payer: &'a AccountView,
    space: usize,
    owner: &Address,
    seeds: &[Seed; N],
) -> ProgramResult {
    let signer_seeds = Signer::from(seeds);

    // Allocate space for account
    let rent = Rent::get()?;

    if target_account.lamports().eq(&0) {
        // If balance is zero, create account

        pinocchio_system::instructions::CreateAccount {
            from: payer,
            to: target_account,
            lamports: rent.try_minimum_balance(space)?,
            space: space as u64,
            owner,
        }
        .invoke_signed(&[signer_seeds.clone()])?;
    } else {
        // Otherwise, if balance is nonzero:

        // 1) transfer sufficient lamports for rent exemption
        let rent_exempt_balance = rent
            .try_minimum_balance(space)?
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
pub fn close_account(account_info: &AccountView, recipient: &AccountView) -> ProgramResult {
    let lamports = account_info.lamports();

    account_info.set_lamports(lamports - lamports);
    recipient.set_lamports(recipient.lamports() + lamports);

    account_info.close()
}
