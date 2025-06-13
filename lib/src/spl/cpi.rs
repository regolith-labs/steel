use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::account::invoke_signed_with_bump;

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
            &token_program.key,
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

#[inline(always)]
pub fn close_token_account<'info>(
    account_info: &AccountInfo<'info>,
    destination_info: &AccountInfo<'info>,
    owner_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::instruction::close_account(
            &token_program.key,
            &account_info.key,
            &destination_info.key,
            &owner_info.key,
            &[&owner_info.key],
        )?,
        &[
            token_program.clone(),
            account_info.clone(),
            destination_info.clone(),
            owner_info.clone(),
        ],
    )
}

#[inline(always)]
pub fn close_token_account_signed<'info>(
    account_info: &AccountInfo<'info>,
    destination_info: &AccountInfo<'info>,
    owner_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    seeds: &[&[u8]],
) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, owner_info.owner).1;
    close_token_account_signed_with_bump(
        account_info,
        destination_info,
        owner_info,
        token_program,
        seeds,
        bump,
    )
}

#[inline(always)]
pub fn close_token_account_signed_with_bump<'info>(
    account_info: &AccountInfo<'info>,
    destination_info: &AccountInfo<'info>,
    owner_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed_with_bump(
        &spl_token_2022::instruction::close_account(
            &token_program.key,
            &account_info.key,
            &destination_info.key,
            &owner_info.key,
            &[&owner_info.key],
        )?,
        &[
            token_program.clone(),
            account_info.clone(),
            destination_info.clone(),
            owner_info.clone(),
        ],
        seeds,
        bump,
    )
}

#[inline(always)]
pub fn transfer<'info>(
    authority_info: &AccountInfo<'info>,
    from_info: &AccountInfo<'info>,
    to_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
) -> ProgramResult {
    #[allow(deprecated)]
    solana_program::program::invoke(
        &spl_token_2022::instruction::transfer(
            &token_program.key,
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
    #[allow(deprecated)]
    invoke_signed_with_bump(
        &spl_token_2022::instruction::transfer(
            &token_program.key,
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

#[inline(always)]
pub fn transfer_checked<'info>(
    authority_info: &AccountInfo<'info>,
    from_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    to_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::instruction::transfer_checked(
            &token_program.key,
            from_info.key,
            mint_info.key,
            to_info.key,
            authority_info.key,
            &[authority_info.key],
            amount,
            decimals,
        )?,
        &[
            token_program.clone(),
            from_info.clone(),
            mint_info.clone(),
            to_info.clone(),
            authority_info.clone(),
        ],
    )
}

#[inline(always)]
pub fn transfer_checked_signed<'info>(
    authority_info: &AccountInfo<'info>,
    from_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    to_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    decimals: u8,
    seeds: &[&[u8]],
) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, authority_info.owner).1;
    transfer_checked_signed_with_bump(
        authority_info,
        from_info,
        mint_info,
        to_info,
        token_program,
        amount,
        decimals,
        seeds,
        bump,
    )
}

#[inline(always)]
pub fn transfer_checked_signed_with_bump<'info>(
    authority_info: &AccountInfo<'info>,
    from_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    to_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    decimals: u8,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed_with_bump(
        &spl_token_2022::instruction::transfer_checked(
            &token_program.key,
            from_info.key,
            mint_info.key,
            to_info.key,
            authority_info.key,
            &[authority_info.key],
            amount,
            decimals,
        )?,
        &[
            token_program.clone(),
            from_info.clone(),
            mint_info.clone(),
            to_info.clone(),
            authority_info.clone(),
        ],
        seeds,
        bump,
    )
}

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
        &spl_token_2022::instruction::mint_to(
            &token_program.key,
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

#[inline(always)]
pub fn mint_to_checked_signed<'info>(
    mint_info: &AccountInfo<'info>,
    to_info: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    decimals: u8,
    seeds: &[&[u8]],
) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, authority_info.owner).1;
    mint_to_checked_signed_with_bump(
        mint_info,
        to_info,
        authority_info,
        token_program,
        amount,
        decimals,
        seeds,
        bump,
    )
}

#[inline(always)]
pub fn mint_to_checked_signed_with_bump<'info>(
    mint_info: &AccountInfo<'info>,
    to_info: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    decimals: u8,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed_with_bump(
        &spl_token_2022::instruction::mint_to_checked(
            &token_program.key,
            mint_info.key,
            to_info.key,
            authority_info.key,
            &[authority_info.key],
            amount,
            decimals,
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

#[inline(always)]
pub fn burn<'info>(
    token_account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::instruction::burn(
            &token_program.key,
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
        &spl_token_2022::instruction::burn(
            &token_program.key,
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

#[inline(always)]
pub fn burn_checked<'info>(
    token_account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::instruction::burn_checked(
            &token_program.key,
            token_account_info.key,
            mint_info.key,
            authority_info.key,
            &[authority_info.key],
            amount,
            decimals,
        )?,
        &[
            token_program.clone(),
            token_account_info.clone(),
            mint_info.clone(),
            authority_info.clone(),
        ],
    )
}

#[inline(always)]
pub fn burn_checked_signed<'info>(
    token_account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    decimals: u8,
    seeds: &[&[u8]],
) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, authority_info.owner).1;
    burn_checked_signed_with_bump(
        token_account_info,
        mint_info,
        authority_info,
        token_program,
        amount,
        decimals,
        seeds,
        bump,
    )
}

#[inline(always)]
pub fn burn_checked_signed_with_bump<'info>(
    token_account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    decimals: u8,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed_with_bump(
        &spl_token_2022::instruction::burn_checked(
            &token_program.key,
            token_account_info.key,
            mint_info.key,
            authority_info.key,
            &[authority_info.key],
            amount,
            decimals,
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

#[inline(always)]
pub fn freeze<'info>(
    account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    owner_info: &AccountInfo<'info>,
    signer_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::instruction::freeze_account(
            &token_program.key,
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
        &spl_token_2022::instruction::freeze_account(
            &token_program.key,
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
        &spl_token_2022::instruction::initialize_mint(
            &token_program.key,
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

#[inline(always)]
pub fn initialize_mint2<'info>(
    mint_info: &AccountInfo<'info>,
    mint_authority_info: &AccountInfo<'info>,
    freeze_authority_info: Option<&AccountInfo<'info>>,
    token_program: &AccountInfo<'info>,
    decimals: u8,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::instruction::initialize_mint2(
            &token_program.key,
            mint_info.key,
            mint_authority_info.key,
            freeze_authority_info.map(|i| i.key),
            decimals,
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
            mint_authority_info.clone(),
        ],
    )
}

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
        &spl_token_2022::instruction::initialize_mint(
            &token_program.key,
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

/// Thaws a frozen SPL token account
///
#[inline(always)]
pub fn thaw_account<'info>(
    token_account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::instruction::thaw_account(
            &token_program.key,
            token_account_info.key,
            mint_info.key,
            authority_info.key,
            &[authority_info.key],
        )?,
        &[
            token_program.clone(),
            token_account_info.clone(),
            mint_info.clone(),
            authority_info.clone(),
        ],
    )
}

/// Thaws a frozen SPL token account using signed account
///
#[inline(always)]
pub fn thaw_account_signed<'info>(
    token_account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    owner_info: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    seeds: &[&[u8]],
) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, authority_info.owner).1;
    thaw_account_signed_with_bump(
        token_account_info,
        mint_info,
        owner_info,
        authority_info,
        token_program,
        seeds,
        bump,
    )
}

#[inline(always)]
pub fn thaw_account_signed_with_bump<'info>(
    token_account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    owner_info: &AccountInfo<'info>,
    signer_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed_with_bump(
        &spl_token_2022::instruction::thaw_account(
            &token_program.key,
            token_account_info.key,
            mint_info.key,
            owner_info.key,
            &[signer_info.key],
        )?,
        &[
            token_program.clone(),
            token_account_info.clone(),
            mint_info.clone(),
            owner_info.clone(),
            signer_info.clone(),
        ],
        seeds,
        bump,
    )
}

/// Set authority for an SPL token mint
///
#[inline(always)]
pub fn set_authority<'info>(
    account_or_mint: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    new_authority_info: Option<&AccountInfo<'info>>,
    authority_type: spl_token_2022::instruction::AuthorityType,
    token_program: &AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::instruction::set_authority(
            &token_program.key,
            account_or_mint.key,
            new_authority_info.map(|i| i.key),
            authority_type,
            authority_info.key,
            &[authority_info.key],
        )?,
        &[
            token_program.clone(),
            account_or_mint.clone(),
            authority_info.clone(),
        ],
    )
}

/// Set authority using signer seeds
///
#[inline(always)]
pub fn set_authority_signed<'info>(
    account_or_mint: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    new_authority_info: Option<&AccountInfo<'info>>,
    authority_type: spl_token_2022::instruction::AuthorityType,
    token_program: &AccountInfo<'info>,
    seeds: &[&[u8]],
) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, authority_info.owner).1;
    set_authority_signed_with_bump(
        account_or_mint,
        authority_info,
        new_authority_info,
        authority_type,
        token_program,
        seeds,
        bump,
    )
}

#[inline(always)]
pub fn set_authority_signed_with_bump<'info>(
    account_or_mint: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    new_authority_info: Option<&AccountInfo<'info>>,
    authority_type: spl_token_2022::instruction::AuthorityType,
    token_program: &AccountInfo<'info>,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed_with_bump(
        &spl_token_2022::instruction::set_authority(
            &token_program.key,
            account_or_mint.key,
            new_authority_info.map(|i| i.key),
            authority_type,
            authority_info.key,
            &[authority_info.key],
        )?,
        &[
            token_program.clone(),
            account_or_mint.clone(),
            authority_info.clone(),
        ],
        seeds,
        bump,
    )
}

/// Revoke
///
#[inline(always)]
pub fn revoke<'info>(
    source_info: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::instruction::revoke(
            &token_program.key,
            source_info.key,
            authority_info.key,
            &[authority_info.key],
        )?,
        &[
            token_program.clone(),
            source_info.clone(),
            authority_info.clone(),
        ],
    )
}

/// Revoke with signer seeds
///
#[inline(always)]
pub fn revoke_signed<'info>(
    source_info: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    seeds: &[&[u8]],
) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, authority_info.owner).1;
    revoke_signed_with_bump(source_info, authority_info, token_program, seeds, bump)
}

#[inline(always)]
pub fn revoke_signed_with_bump<'info>(
    source_info: &AccountInfo<'info>,
    authority_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed_with_bump(
        &spl_token_2022::instruction::revoke(
            &token_program.key,
            source_info.key,
            authority_info.key,
            &[authority_info.key],
        )?,
        &[
            token_program.clone(),
            source_info.clone(),
            authority_info.clone(),
        ],
        seeds,
        bump,
    )
}

///  /// Approves a delegate.  A delegate is given the authority over tokens on
/// behalf of the source account's owner.
#[inline(always)]
pub fn approve<'info>(
    source_info: &AccountInfo<'info>,
    delegate_info: &AccountInfo<'info>,
    owner_info: &AccountInfo<'info>,
    signer_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token::instruction::approve(
            &spl_token::ID,
            source_info.key,
            delegate_info.key,
            owner_info.key,
            &[signer_info.key],
            amount,
        )?,
        &[
            token_program.clone(),
            source_info.clone(),
            delegate_info.clone(),
            owner_info.clone(),
            signer_info.clone(),
        ],
    )
}

#[inline(always)]
pub fn approve_signed<'info>(
    source_info: &AccountInfo<'info>,
    delegate_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    seeds: &[&[u8]],
) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, &spl_token::ID).1;
    approve_signed_with_bump(
        source_info,
        delegate_info,
        token_program,
        amount,
        seeds,
        bump,
    )
}

#[inline(always)]
pub fn approve_signed_with_bump<'info>(
    source_info: &AccountInfo<'info>,
    delegate_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed_with_bump(
        &spl_token_2022::instruction::approve(
            &token_program.key,
            source_info.key,
            delegate_info.key,
            delegate_info.key,
            &[delegate_info.key],
            amount,
        )?,
        &[
            token_program.clone(),
            source_info.clone(),
            delegate_info.clone(),
        ],
        seeds,
        bump,
    )
}

#[inline(always)]
pub fn approve_checked<'info>(
    source_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    delegate_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::instruction::approve_checked(
            &token_program.key,
            mint_info.key,
            source_info.key,
            delegate_info.key,
            delegate_info.key,
            &[delegate_info.key],
            amount,
            decimals,
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
            source_info.clone(),
            delegate_info.clone(),
        ],
    )
}

#[inline(always)]
pub fn approve_checked_signed<'info>(
    source_info: &AccountInfo<'info>,
    delegate_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
    seeds: &[&[u8]],
) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, &spl_token::ID).1;
    approve_signed_with_bump(
        source_info,
        delegate_info,
        token_program,
        amount,
        seeds,
        bump,
    )
}

#[inline(always)]
pub fn approve_checked_signed_with_bump<'info>(
    source_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    delegate_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    decimals: u8,
    amount: u64,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed_with_bump(
        &spl_token_2022::instruction::approve_checked(
            &token_program.key,
            source_info.key,
            mint_info.key,
            delegate_info.key,
            delegate_info.key,
            &[delegate_info.key],
            amount,
            decimals,
        )?,
        &[source_info.clone(), delegate_info.clone()],
        seeds,
        bump,
    )
}

/// Initializes a multisignature account with N provided signers.
#[inline(always)]
pub fn initialize_multisig<'info>(
    multisig_info: &AccountInfo<'info>,
    signer_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    n: u8,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::instruction::initialize_multisig(
            &token_program.key,
            multisig_info.key,
            &[signer_info.key],
            n,
        )?,
        &[token_program.clone(), signer_info.clone()],
    )
}

#[inline(always)]
pub fn initialize_multisig_signed<'info>(
    multisig_info: &AccountInfo<'info>,
    signer_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    m: u8, // number of multi signtures
    seeds: &[&[u8]],
) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, &spl_token::ID).1;
    initialize_multisig_signed_with_bump(multisig_info, signer_info, token_program, m, seeds, bump)
}

#[inline(always)]
pub fn initialize_multisig_signed_with_bump<'info>(
    multisig_info: &AccountInfo<'info>,
    signer_info: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    m: u8,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed_with_bump(
        &spl_token_2022::instruction::initialize_multisig(
            &token_program.key,
            multisig_info.key,
            &[signer_info.key],
            m,
        )?,
        &[token_program.clone(), signer_info.clone()],
        seeds,
        bump,
    )
}

///Like InitializeAccount2, but does not require the Rent sysvar to be provided
#[inline(always)]
pub fn sync_native(account_info: &AccountInfo) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::instruction::sync_native(account_info.owner, account_info.key)?,
        &[account_info.clone()],
    )
}

#[inline(always)]
pub fn sync_native_signed(account_info: &AccountInfo, seeds: &[&[u8]]) -> ProgramResult {
    let bump = Pubkey::find_program_address(seeds, &spl_token::ID).1;
    sync_native_signed_with_bump(account_info, seeds, bump)
}

#[inline(always)]
pub fn sync_native_signed_with_bump(
    account_info: &AccountInfo,
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed_with_bump(
        &spl_token_2022::instruction::sync_native(account_info.owner, account_info.key)?,
        &[account_info.clone()],
        seeds,
        bump,
    )
}

#[inline(always)]
pub fn get_account_data_size(
    mint_info: &AccountInfo,
    extension_types: &[spl_token_2022::extension::ExtensionType],
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::instruction::get_account_data_size(
            mint_info.owner,
            mint_info.key,
            extension_types,
        )?,
        &[mint_info.clone()],
    )
}
