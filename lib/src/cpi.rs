use bytemuck::Pod;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, instruction::Instruction, pubkey::Pubkey,
    rent::Rent, sysvar::Sysvar,
};

use crate::Discriminator;

/// Creates a new program account.
#[inline(always)]
pub fn create_account<'a, 'info, T: Discriminator + Pod>(
    target_account: &'a AccountInfo<'info>,
    system_program: &'a AccountInfo<'info>,
    payer: &'a AccountInfo<'info>,
    owner: &Pubkey,
    seeds: &[&[u8]],
) -> ProgramResult {
    create_account_with_bump::<T>(
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
pub fn create_account_with_bump<'a, 'info, T: Discriminator + Pod>(
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
    // Realloc data to zero.
    account_info.realloc(0, true)?;

    // Return rent lamports.
    **recipient.lamports.borrow_mut() += account_info.lamports();
    **account_info.lamports.borrow_mut() = 0;

    Ok(())
}

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
    seeds: &[&[u8]],
) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, authority_info.owner).1;
    transfer_signed_with_bump(
        authority_info,
        from_info,
        to_info,
        token_program,
        amount,
        seeds,
        bump,
    )
}

#[cfg(feature = "spl")]
#[inline(always)]
pub fn transfer_signed_with_bump<'info>(
    authority_info: &AccountInfo<'info>,
    from_info: &AccountInfo<'info>,
    to_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed_with_bump(
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
        seeds,
        bump,
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
    seeds: &[&[u8]],
) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, authority_info.owner).1;
    mint_to_signed_with_bump(
        mint_info,
        to_info,
        authority_info,
        token_program,
        amount,
        seeds,
        bump,
    )
}

#[cfg(feature = "spl")]
#[inline(always)]
pub fn mint_to_signed_with_bump<'info>(
    mint_info: &AccountInfo<'info>,
    to_info: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed_with_bump(
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
        seeds,
        bump,
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

#[cfg(feature = "spl")]
#[inline(always)]
pub fn burn_signed<'info>(
    token_account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    seeds: &[&[u8]],
) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, authority_info.owner).1;
    burn_signed_with_bump(
        token_account_info,
        mint_info,
        authority_info,
        token_program,
        amount,
        seeds,
        bump,
    )
}

#[cfg(feature = "spl")]
#[inline(always)]
pub fn burn_signed_with_bump<'info>(
    token_account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed_with_bump(
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
        seeds,
        bump,
    )
}

#[cfg(feature = "spl")]
#[inline(always)]
pub fn freeze<'info>(
    account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    owner_info: &AccountInfo<'info>,
    signer_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token::instruction::freeze_account(
            &spl_token::ID,
            account_info.key,
            mint_info.key,
            owner_info.key,
            &[signer_info.key],
        )?,
        &[
            token_program.clone(),
            account_info.clone(),
            mint_info.clone(),
            owner_info.clone(),
            signer_info.clone(),
        ],
    )
}

#[cfg(feature = "spl")]
#[inline(always)]
pub fn freeze_signed<'info>(
    account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    owner_info: &AccountInfo<'info>,
    signer_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    seeds: &[&[u8]],
) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, signer_info.owner).1;
    freeze_signed_with_bump(
        account_info,
        mint_info,
        owner_info,
        signer_info,
        token_program,
        seeds,
        bump,
    )
}

#[cfg(feature = "spl")]
#[inline(always)]
pub fn freeze_signed_with_bump<'info>(
    account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    owner_info: &AccountInfo<'info>,
    signer_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed_with_bump(
        &spl_token::instruction::freeze_account(
            &spl_token::ID,
            account_info.key,
            mint_info.key,
            owner_info.key,
            &[signer_info.key],
        )?,
        &[
            token_program.clone(),
            account_info.clone(),
            mint_info.clone(),
            owner_info.clone(),
            signer_info.clone(),
        ],
        seeds,
        bump,
    )
}

#[cfg(feature = "spl")]
#[inline(always)]
pub fn initialize_mint<'info>(
    mint_info: &AccountInfo<'info>,
    mint_authority_info: &AccountInfo<'info>,
    freeze_authority_info: Option<&AccountInfo<'info>>,
    token_program: &AccountInfo<'info>,
    rent_sysvar: &AccountInfo<'info>,
    decimals: u8,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token::instruction::initialize_mint(
            &spl_token::ID,
            mint_info.key,
            mint_authority_info.key,
            freeze_authority_info.map(|i| i.key),
            decimals,
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
            mint_authority_info.clone(),
            rent_sysvar.clone(),
        ],
    )
}

#[cfg(feature = "spl")]
#[inline(always)]
pub fn initialize_mint_signed<'info>(
    mint_info: &AccountInfo<'info>,
    mint_authority_info: &AccountInfo<'info>,
    freeze_authority_info: Option<&AccountInfo<'info>>,
    token_program: &AccountInfo<'info>,
    rent_sysvar: &AccountInfo<'info>,
    decimals: u8,
    seeds: &[&[u8]],
) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, mint_info.owner).1;
    initialize_mint_signed_with_bump(
        mint_info,
        mint_authority_info,
        freeze_authority_info,
        token_program,
        rent_sysvar,
        decimals,
        seeds,
        bump,
    )
}

#[cfg(feature = "spl")]
#[inline(always)]
pub fn initialize_mint_signed_with_bump<'info>(
    mint_info: &AccountInfo<'info>,
    mint_authority_info: &AccountInfo<'info>,
    freeze_authority_info: Option<&AccountInfo<'info>>,
    token_program: &AccountInfo<'info>,
    rent_sysvar: &AccountInfo<'info>,
    decimals: u8,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed_with_bump(
        &spl_token::instruction::initialize_mint(
            &spl_token::ID,
            mint_info.key,
            mint_authority_info.key,
            freeze_authority_info.map(|i| i.key),
            decimals,
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
            mint_authority_info.clone(),
            rent_sysvar.clone(),
        ],
        seeds,
        bump,
    )
}
