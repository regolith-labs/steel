use bytemuck::Pod;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, instruction::Instruction, pubkey::Pubkey,
    rent::Rent, sysvar::Sysvar,
};

use crate::{CloseAccount, Discriminator};

/// Invokes a CPI with provided signer seeds and program id.
#[inline(always)]
pub fn invoke_signed<'info>(
    instruction: &Instruction,
    account_infos: &[AccountInfo<'info>],
    program_id: &Pubkey,
    seeds: &[&[u8]],
) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, program_id).1;
    invoke_signed_with_bump(instruction, account_infos, seeds, bump)
}

/// Invokes a CPI with the provided signer seeds and bump.
#[inline(always)]
pub fn invoke_signed_with_bump<'info>(
    instruction: &Instruction,
    account_infos: &[AccountInfo<'info>],
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    // Combine seeds
    let bump: &[u8] = &[bump];
    let mut combined_seeds = Vec::with_capacity(seeds.len() + 1);
    combined_seeds.extend_from_slice(seeds);
    combined_seeds.push(bump);
    let seeds = combined_seeds.as_slice();

    // Invoke CPI
    solana_program::program::invoke_signed(instruction, account_infos, &[seeds])
}

/// Creates a new account.
#[inline(always)]
pub fn create_account<'a, 'info>(
    from_pubkey: &'a AccountInfo<'info>,
    to_pubkey: &'a AccountInfo<'info>,
    system_program: &'a AccountInfo<'info>,
    space: usize,
    owner: &Pubkey,
) -> ProgramResult {
    let lamports_required = (Rent::get()?).minimum_balance(space);

    solana_program::program::invoke(
        &solana_program::system_instruction::create_account(
            from_pubkey.key,
            to_pubkey.key,
            lamports_required,
            space as u64,
            owner,
        ),
        &[
            from_pubkey.clone(),
            to_pubkey.clone(),
            system_program.clone(),
        ],
    )?;

    Ok(())
}

/// Creates a new program account.
#[inline(always)]
pub fn create_program_account<'a, 'info, T: Discriminator + Pod>(
    target_account: &'a AccountInfo<'info>,
    system_program: &'a AccountInfo<'info>,
    payer: &'a AccountInfo<'info>,
    owner: &Pubkey,
    seeds: &[&[u8]],
) -> ProgramResult {
    create_program_account_with_bump::<T>(
        target_account,
        system_program,
        payer,
        owner,
        seeds,
        Pubkey::find_program_address(seeds, owner).1,
    )
}

/// Creates a new program account with user-provided bump.
#[inline(always)]
pub fn create_program_account_with_bump<'a, 'info, T: Discriminator + Pod>(
    target_account: &'a AccountInfo<'info>,
    system_program: &'a AccountInfo<'info>,
    payer: &'a AccountInfo<'info>,
    owner: &Pubkey,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    // Allocate space.
    allocate_account_with_bump(
        target_account,
        system_program,
        payer,
        8 + std::mem::size_of::<T>(),
        owner,
        seeds,
        bump,
    )?;

    // Set discriminator.
    let mut data = target_account.data.borrow_mut();
    data[0] = T::discriminator();

    Ok(())
}

/// Allocates space for a new program account.
#[inline(always)]
pub fn allocate_account<'a, 'info>(
    target_account: &'a AccountInfo<'info>,
    system_program: &'a AccountInfo<'info>,
    payer: &'a AccountInfo<'info>,
    space: usize,
    owner: &Pubkey,
    seeds: &[&[u8]],
) -> ProgramResult {
    allocate_account_with_bump(
        target_account,
        system_program,
        payer,
        space,
        owner,
        seeds,
        Pubkey::find_program_address(seeds, owner).1,
    )
}

/// Allocates space for a new program account with user-provided bump.
#[inline(always)]
pub fn allocate_account_with_bump<'a, 'info>(
    target_account: &'a AccountInfo<'info>,
    system_program: &'a AccountInfo<'info>,
    payer: &'a AccountInfo<'info>,
    space: usize,
    owner: &Pubkey,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    // Combine seeds
    let bump: &[u8] = &[bump];
    let mut combined_seeds = Vec::with_capacity(seeds.len() + 1);
    combined_seeds.extend_from_slice(seeds);
    combined_seeds.push(bump);
    let seeds = combined_seeds.as_slice();

    // Allocate space for account
    let rent = Rent::get()?;
    if target_account.lamports().eq(&0) {
        // If balance is zero, create account
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::create_account(
                payer.key,
                target_account.key,
                rent.minimum_balance(space),
                space as u64,
                owner,
            ),
            &[
                payer.clone(),
                target_account.clone(),
                system_program.clone(),
            ],
            &[seeds],
        )?;
    } else {
        // Otherwise, if balance is nonzero:

        // 1) transfer sufficient lamports for rent exemption
        let rent_exempt_balance = rent
            .minimum_balance(space)
            .saturating_sub(target_account.lamports());
        if rent_exempt_balance.gt(&0) {
            solana_program::program::invoke(
                &solana_program::system_instruction::transfer(
                    payer.key,
                    target_account.key,
                    rent_exempt_balance,
                ),
                &[
                    payer.clone(),
                    target_account.clone(),
                    system_program.clone(),
                ],
            )?;
        }

        // 2) allocate space for the account
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::allocate(target_account.key, space as u64),
            &[target_account.clone(), system_program.clone()],
            &[seeds],
        )?;

        // 3) assign our program as the owner
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::assign(target_account.key, owner),
            &[target_account.clone(), system_program.clone()],
            &[seeds],
        )?;
    }

    Ok(())
}

/// Closes an account and returns the remaining rent lamports to the provided recipient.
#[inline(always)]
pub fn close_account<'info>(
    account_info: &AccountInfo<'info>,
    recipient: &AccountInfo<'info>,
) -> ProgramResult {
    account_info.close(recipient)
}
