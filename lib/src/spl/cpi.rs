use pinocchio::{
    cpi::{Seed, Signer},
    AccountView, Address, ProgramResult,
};

#[inline(always)]
pub fn create_associated_token_account(
    funder_info: &AccountView,
    owner_info: &AccountView,
    token_account_info: &AccountView,
    mint_info: &AccountView,
    system_program: &AccountView,
    token_program: &AccountView,
) -> ProgramResult {
    pinocchio_associated_token_account::instructions::Create {
        funding_account: funder_info,
        account: token_account_info,
        wallet: owner_info,
        mint: mint_info,
        system_program,
        token_program,
    }
    .invoke()
}

// TODO: why is this using token 2022?
#[inline(always)]
pub fn close_token_account(
    account_info: &AccountView,
    destination_info: &AccountView,
    owner_info: &AccountView,
    token_program: &Address,
) -> ProgramResult {
    pinocchio_token_2022::instructions::CloseAccount {
        account: account_info,
        destination: destination_info,
        authority: owner_info,
        token_program: token_program,
    }
    .invoke()
}

#[inline(always)]
pub fn close_token_account_signed<const N: usize>(
    account_info: &AccountView,
    destination_info: &AccountView,
    owner_info: &AccountView,
    token_program: &Address,
    seeds: &[Seed; N],
) -> ProgramResult {
    let signer_seeds = Signer::from(seeds);

    pinocchio_token_2022::instructions::CloseAccount {
        account: account_info,
        destination: destination_info,
        authority: owner_info,
        token_program: token_program,
    }
    .invoke_signed(&[signer_seeds])
}

#[inline(always)]
pub fn transfer(
    authority_info: &AccountView,
    from_info: &AccountView,
    to_info: &AccountView,
    token_program: &Address,
    amount: u64,
) -> ProgramResult {
    pinocchio_token_2022::instructions::Transfer {
        from: from_info,
        to: to_info,
        authority: authority_info,
        amount: amount,
        token_program: token_program,
    }
    .invoke()
}

#[inline(always)]
pub fn transfer_signed<const N: usize>(
    authority_info: &AccountView,
    from_info: &AccountView,
    to_info: &AccountView,
    token_program: &Address,
    amount: u64,
    seeds: &[Seed; N],
) -> ProgramResult {
    let signer_seeds = Signer::from(seeds);

    pinocchio_token_2022::instructions::Transfer {
        from: from_info,
        to: to_info,
        authority: authority_info,
        amount: amount,
        token_program: token_program,
    }
    .invoke_signed(&[signer_seeds])
}

#[inline(always)]
pub fn transfer_checked(
    authority_info: &AccountView,
    from_info: &AccountView,
    mint_info: &AccountView,
    to_info: &AccountView,
    token_program: &Address,
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    pinocchio_token_2022::instructions::TransferChecked {
        from: from_info,
        mint: mint_info,
        to: to_info,
        token_program: token_program,
        authority: authority_info,
        amount,
        decimals,
    }
    .invoke()
}

#[inline(always)]
pub fn transfer_checked_signed<const N: usize>(
    authority_info: &AccountView,
    from_info: &AccountView,
    mint_info: &AccountView,
    to_info: &AccountView,
    token_program: &Address,
    amount: u64,
    decimals: u8,
    seeds: &[Seed; N],
) -> ProgramResult {
    let signer_seeds = Signer::from(seeds);

    pinocchio_token_2022::instructions::TransferChecked {
        from: from_info,
        mint: mint_info,
        to: to_info,
        token_program: token_program,
        authority: authority_info,
        amount,
        decimals,
    }
    .invoke_signed(&[signer_seeds])
}

#[inline(always)]
pub fn mint_to_signed<const N: usize>(
    mint_info: &AccountView,
    to_info: &AccountView,
    authority_info: &AccountView,
    amount: u64,
    seeds: &[Seed; N],
) -> ProgramResult {
    let signer_seeds = Signer::from(seeds);

    pinocchio_token::instructions::MintTo {
        mint: mint_info,
        account: to_info,
        mint_authority: authority_info,
        amount,
    }
    .invoke_signed(&[signer_seeds])
}

#[inline(always)]
pub fn mint_to_checked_signed<const N: usize>(
    mint_info: &AccountView,
    to_info: &AccountView,
    authority_info: &AccountView,
    amount: u64,
    decimals: u8,
    seeds: &[Seed; N],
) -> ProgramResult {
    let signer_seeds = Signer::from(seeds);

    pinocchio_token::instructions::MintToChecked {
        mint: mint_info,
        account: to_info,
        mint_authority: authority_info,
        amount,
        decimals,
    }
    .invoke_signed(&[signer_seeds])
}

#[inline(always)]
pub fn burn(
    token_account_info: &AccountView,
    mint_info: &AccountView,
    authority_info: &AccountView,
    token_program: &Address,
    amount: u64,
) -> ProgramResult {
    pinocchio_token_2022::instructions::Burn {
        account: token_account_info,
        mint: mint_info,
        authority: authority_info,
        amount,
        token_program,
    }
    .invoke()
}

#[inline(always)]
pub fn burn_signed<const N: usize>(
    token_account_info: &AccountView,
    mint_info: &AccountView,
    authority_info: &AccountView,
    token_program: &Address,
    amount: u64,
    seeds: &[Seed; N],
) -> ProgramResult {
    let signer_seeds = Signer::from(seeds);

    pinocchio_token_2022::instructions::Burn {
        account: token_account_info,
        mint: mint_info,
        authority: authority_info,
        amount,
        token_program,
    }
    .invoke_signed(&[signer_seeds])
}

#[inline(always)]
pub fn burn_checked(
    token_account_info: &AccountView,
    mint_info: &AccountView,
    authority_info: &AccountView,
    token_program: &Address,
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    pinocchio_token_2022::instructions::BurnChecked {
        account: token_account_info,
        mint: mint_info,
        authority: authority_info,
        amount,
        decimals,
        token_program,
    }
    .invoke()
}

#[inline(always)]
pub fn burn_checked_signed<const N: usize>(
    token_account_info: &AccountView,
    mint_info: &AccountView,
    authority_info: &AccountView,
    token_program: &Address,
    amount: u64,
    decimals: u8,
    seeds: &[Seed; N],
) -> ProgramResult {
    let signer_seeds = Signer::from(seeds);

    pinocchio_token_2022::instructions::BurnChecked {
        account: token_account_info,
        mint: mint_info,
        authority: authority_info,
        amount,
        decimals,
        token_program,
    }
    .invoke_signed(&[signer_seeds])
}

#[inline(always)]
pub fn freeze(
    account_info: &AccountView,
    mint_info: &AccountView,
    authority_info: &AccountView,
    token_program: &Address,
) -> ProgramResult {
    pinocchio_token_2022::instructions::FreezeAccount {
        account: account_info,
        mint: mint_info,
        freeze_authority: authority_info,
        token_program,
    }
    .invoke()
}

#[inline(always)]
pub fn freeze_signed<const N: usize>(
    account_info: &AccountView,
    mint_info: &AccountView,
    authority_info: &AccountView,
    token_program: &Address,
    seeds: &[Seed; N],
) -> ProgramResult {
    let signer_seeds = Signer::from(seeds);

    pinocchio_token_2022::instructions::FreezeAccount {
        account: account_info,
        mint: mint_info,
        freeze_authority: authority_info,
        token_program,
    }
    .invoke_signed(&[signer_seeds])
}

#[inline(always)]
pub fn initialize_mint(
    mint_info: &AccountView,
    rent_sysvar: &AccountView,
    mint_authority: &Address,
    freeze_authority: Option<&Address>,
    token_program: &Address,
    decimals: u8,
) -> ProgramResult {
    pinocchio_token_2022::instructions::InitializeMint {
        mint: mint_info,
        rent_sysvar,
        decimals,
        mint_authority,
        freeze_authority,
        token_program,
    }
    .invoke()
}

/// Thaws a frozen SPL token account
#[inline(always)]
pub fn thaw_account(
    token_account_info: &AccountView,
    mint_info: &AccountView,
    authority_info: &AccountView,
    token_program: &Address,
) -> ProgramResult {
    pinocchio_token_2022::instructions::ThawAccount {
        account: token_account_info,
        mint: mint_info,
        freeze_authority: authority_info,
        token_program,
    }
    .invoke()
}

/// Thaws a frozen SPL token account using signed account
#[inline(always)]
pub fn thaw_account_signed<const N: usize>(
    token_account_info: &AccountView,
    mint_info: &AccountView,
    authority_info: &AccountView,
    token_program: &Address,
    seeds: &[Seed; N],
) -> ProgramResult {
    let signer_seeds = Signer::from(seeds);

    pinocchio_token_2022::instructions::ThawAccount {
        account: token_account_info,
        mint: mint_info,
        freeze_authority: authority_info,
        token_program,
    }
    .invoke_signed(&[signer_seeds])
}

/// Set authority for an SPL token mint
#[inline(always)]
pub fn set_authority(
    account_or_mint: &AccountView,
    authority_info: &AccountView,
    new_authority: Option<&Address>,
    authority_type: pinocchio_token_2022::instructions::AuthorityType,
    token_program: &Address,
) -> ProgramResult {
    pinocchio_token_2022::instructions::SetAuthority {
        account: account_or_mint,
        authority: authority_info,
        authority_type,
        new_authority,
        token_program,
    }
    .invoke()
}

/// Set authority using signer seeds
#[inline(always)]
pub fn set_authority_signed<const N: usize>(
    account_or_mint: &AccountView,
    authority_info: &AccountView,
    new_authority: Option<&Address>,
    authority_type: pinocchio_token_2022::instructions::AuthorityType,
    token_program: &Address,
    seeds: &[Seed; N],
) -> ProgramResult {
    let signer_seeds = Signer::from(seeds);

    pinocchio_token_2022::instructions::SetAuthority {
        account: account_or_mint,
        authority: authority_info,
        authority_type,
        new_authority,
        token_program,
    }
    .invoke_signed(&[signer_seeds])
}

/// Revoke
#[inline(always)]
pub fn revoke(
    source_info: &AccountView,
    authority_info: &AccountView,
    token_program: &Address,
) -> ProgramResult {
    pinocchio_token_2022::instructions::Revoke {
        source: source_info,
        authority: authority_info,
        token_program,
    }
    .invoke()
}

/// Revoke with signer seeds
#[inline(always)]
pub fn revoke_signed<const N: usize>(
    source_info: &AccountView,
    authority_info: &AccountView,
    token_program: &Address,
    seeds: &[Seed; N],
) -> ProgramResult {
    let signer_seeds = Signer::from(seeds);

    pinocchio_token_2022::instructions::Revoke {
        source: source_info,
        authority: authority_info,
        token_program,
    }
    .invoke_signed(&[signer_seeds])
}

/// Approves a delegate. A delegate is given the authority over tokens on
/// behalf of the source account's owner.
#[inline(always)]
pub fn approve(
    source_info: &AccountView,
    delegate_info: &AccountView,
    authority_info: &AccountView,
    token_program: &Address,
    amount: u64,
) -> ProgramResult {
    pinocchio_token_2022::instructions::Approve {
        source: source_info,
        delegate: delegate_info,
        authority: authority_info,
        amount,
        token_program,
    }
    .invoke()
}

#[inline(always)]
pub fn approve_signed<const N: usize>(
    source_info: &AccountView,
    delegate_info: &AccountView,
    authority_info: &AccountView,
    token_program: &Address,
    amount: u64,
    seeds: &[Seed; N],
) -> ProgramResult {
    let signer_seeds = Signer::from(seeds);

    pinocchio_token_2022::instructions::Approve {
        source: source_info,
        delegate: delegate_info,
        authority: authority_info,
        amount,
        token_program,
    }
    .invoke_signed(&[signer_seeds])
}

#[inline(always)]
pub fn approve_checked(
    source_info: &AccountView,
    mint_info: &AccountView,
    delegate_info: &AccountView,
    authority_info: &AccountView,
    token_program: &Address,
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    pinocchio_token_2022::instructions::ApproveChecked {
        source: source_info,
        mint: mint_info,
        delegate: delegate_info,
        authority: authority_info,
        amount,
        decimals,
        token_program,
    }
    .invoke()
}

#[inline(always)]
pub fn approve_checked_signed<const N: usize>(
    source_info: &AccountView,
    mint_info: &AccountView,
    delegate_info: &AccountView,
    authority_info: &AccountView,
    token_program: &Address,
    amount: u64,
    decimals: u8,
    seeds: &[Seed; N],
) -> ProgramResult {
    let signer_seeds = Signer::from(seeds);

    pinocchio_token_2022::instructions::ApproveChecked {
        source: source_info,
        mint: mint_info,
        delegate: delegate_info,
        authority: authority_info,
        amount,
        decimals,
        token_program,
    }
    .invoke_signed(&[signer_seeds])
}

/// Initializes a multisignature account with N provided signers.
#[inline(always)]
pub fn initialize_multisig<'a, const S: usize>(
    multisig_info: &AccountView,
    rent_sysvar: &AccountView,
    signers: &[&'a AccountView; S],
    token_program: &Address,
    m: u8,
) -> ProgramResult {
    pinocchio_token_2022::instructions::InitializeMultisig {
        multisig: multisig_info,
        rent_sysvar,
        signers,
        token_program,
        m,
    }
    .invoke()
}

/// Like InitializeAccount2, but does not require the Rent sysvar to be provided
#[inline(always)]
pub fn sync_native(account_info: &AccountView, token_program: &Address) -> ProgramResult {
    pinocchio_token_2022::instructions::SyncNative {
        native_token: account_info,
        token_program,
    }
    .invoke()
}
