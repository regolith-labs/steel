use bytemuck::Pod;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey, rent::Rent,
    sysvar::Sysvar,
};

use crate::Discriminator;

/// Creates a new program account.
#[inline(always)]
pub fn create_account<'a, 'info, T: Discriminator + Pod>(
    target_account: &'a AccountInfo<'info>,
    owner: &Pubkey,
    seeds: &[&[u8]],
    system_program: &'a AccountInfo<'info>,
    payer: &'a AccountInfo<'info>,
) -> ProgramResult {
    create_account_with_bump::<T>(
        target_account,
        owner,
        seeds,
        Pubkey::find_program_address(seeds, owner).1,
        system_program,
        payer,
    )
}

/// Creates a new program account with user-provided bump.
#[inline(always)]
pub fn create_account_with_bump<'a, 'info, T: Discriminator + Pod>(
    target_account: &'a AccountInfo<'info>,
    owner: &Pubkey,
    seeds: &[&[u8]],
    bump: u8,
    system_program: &'a AccountInfo<'info>,
    payer: &'a AccountInfo<'info>,
) -> ProgramResult {
    // Allocate space.
    allocate_account_with_bump(
        target_account,
        owner,
        8 + std::mem::size_of::<T>(),
        seeds,
        bump,
        system_program,
        payer,
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
    owner: &Pubkey,
    space: usize,
    seeds: &[&[u8]],
    system_program: &'a AccountInfo<'info>,
    payer: &'a AccountInfo<'info>,
) -> ProgramResult {
    allocate_account_with_bump(
        target_account,
        owner,
        space,
        seeds,
        Pubkey::find_program_address(seeds, owner).1,
        system_program,
        payer,
    )
}

/// Allocates space for a new program account with user-provided bump.
#[inline(always)]
pub fn allocate_account_with_bump<'a, 'info>(
    target_account: &'a AccountInfo<'info>,
    owner: &Pubkey,
    space: usize,
    seeds: &[&[u8]],
    bump: u8,
    system_program: &'a AccountInfo<'info>,
    payer: &'a AccountInfo<'info>,
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

#[cfg(feature = "spl")]
#[inline(always)]
pub fn create_associated_token_account<'info>(
    funder_info: &AccountInfo<'info>,
    owner_info: &AccountInfo<'info>,
    token_account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    associated_token_program: &AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_associated_token_account::instruction::create_associated_token_account(
            funder_info.key,
            owner_info.key,
            mint_info.key,
            &spl_token::ID,
        ),
        &[
            funder_info.clone(),
            token_account_info.clone(),
            owner_info.clone(),
            mint_info.clone(),
            system_program.clone(),
            token_program.clone(),
            associated_token_program.clone(),
        ],
    )
}

#[cfg(feature = "spl")]
#[inline(always)]
pub fn transfer<'info>(
    authority_info: &AccountInfo<'info>,
    from_info: &AccountInfo<'info>,
    to_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token::instruction::transfer(
            &spl_token::ID,
            from_info.key,
            to_info.key,
            authority_info.key,
            &[authority_info.key],
            amount,
        )?,
        &[
            token_program.clone(),
            from_info.clone(),
            to_info.clone(),
            authority_info.clone(),
        ],
    )
}

#[cfg(feature = "spl")]
#[inline(always)]
pub fn transfer_signed<'info>(
    authority_info: &AccountInfo<'info>,
    from_info: &AccountInfo<'info>,
    to_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    solana_program::program::invoke_signed(
        &spl_token::instruction::transfer(
            &spl_token::ID,
            from_info.key,
            to_info.key,
            authority_info.key,
            &[authority_info.key],
            amount,
        )?,
        &[
            token_program.clone(),
            from_info.clone(),
            to_info.clone(),
            authority_info.clone(),
        ],
        signer_seeds,
    )
}

#[cfg(feature = "spl")]
#[inline(always)]
pub fn mint_to_signed<'info>(
    mint_info: &AccountInfo<'info>,
    to_info: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    solana_program::program::invoke_signed(
        &spl_token::instruction::mint_to(
            &spl_token::ID,
            mint_info.key,
            to_info.key,
            authority_info.key,
            &[authority_info.key],
            amount,
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
            to_info.clone(),
            authority_info.clone(),
        ],
        signer_seeds,
    )
}

#[cfg(feature = "spl")]
#[inline(always)]
pub fn burn<'info>(
    token_account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token::instruction::burn(
            &spl_token::ID,
            token_account_info.key,
            mint_info.key,
            authority_info.key,
            &[authority_info.key],
            amount,
        )?,
        &[
            token_program.clone(),
            token_account_info.clone(),
            mint_info.clone(),
            authority_info.clone(),
        ],
    )
}
