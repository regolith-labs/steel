use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};
use spl_token_2022::solana_zk_token_sdk::zk_token_elgamal::pod::ElGamalPubkey;
use spl_token_2022::state::AccountState;


//EXTENSION INSTRUCTIONS
//TRANSFER FEE EXTENSION
#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn set_transfer_fee<'a, 'info>(
    mint_info: &'a AccountInfo<'info>,
    authority_info: &'a AccountInfo<'info>,
    signer_info: &'a AccountInfo<'info>,
    token_program: &'a AccountInfo<'info>,
    transfer_fee_basis_points: u16,
    maximum_fee: u64,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::transfer_fee::instruction::set_transfer_fee(
            &spl_token_2022::id(),
            mint_info.key,
            authority_info.key,
            &[signer_info.key],
            transfer_fee_basis_points,
            maximum_fee,
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
            authority_info.clone(),
            signer_info.clone(),
        ],
    )?;

    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn harvest_withheld_tokens_to_mint<'a, 'info>(
    mint_info: &'a AccountInfo<'info>,
    token_program: &'a AccountInfo<'info>,
    source_infos: &[&'a AccountInfo<'info>],
) -> ProgramResult {
    let sources: Vec<&Pubkey> = source_infos.iter().map(|signer| signer.key).collect();
    let mut account_infos = vec![token_program.clone(), mint_info.clone()];
    account_infos.extend(source_infos.iter().map(|s| (*s).clone()));

    solana_program::program::invoke(
        &spl_token_2022::extension::transfer_fee::instruction::harvest_withheld_tokens_to_mint(
            &spl_token_2022::id(),
            mint_info.key,
            &sources,
        )?,
        &account_infos,
    )?;
    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn transfer_checked_with_fee<'a, 'info>(
    mint_info: &'a AccountInfo<'info>,
    source_info: &'a AccountInfo<'info>,
    destination_info: &'a AccountInfo<'info>,
    authority_info: &'a AccountInfo<'info>,
    signer_info: AccountInfo<'info>,
    token_program: &'a AccountInfo<'info>,
    amount: u64,
    decimals: u8,
    fee: u64,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::transfer_fee::instruction::transfer_checked_with_fee(
            &spl_token_2022::id(),
            source_info.key,
            mint_info.key,
            destination_info.key,
            authority_info.key,
            &[signer_info.key],
            amount,
            decimals,
            fee,
        )?,
        &[
            token_program.clone(),
            source_info.clone(),
            mint_info.clone(),
            destination_info.clone(),
            authority_info.clone(),
            signer_info.clone(),
        ], //add signers
    )?;

    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn initialize_transfer_fee_config<'a, 'info>(
    mint_info: &'a AccountInfo<'info>,
    transfer_fee_config_authority_info: Option<&'a AccountInfo<'info>>,
    withdraw_withheld_authority_info: Option<&'a AccountInfo<'info>>,
    token_program: &'a AccountInfo<'info>,
    transfer_fee_basis_points: u16,
    maximum_fee: u64,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::transfer_fee::instruction::initialize_transfer_fee_config(
            &spl_token_2022::id(),
            mint_info.key,
            transfer_fee_config_authority_info.map(|acc| acc.key),
            withdraw_withheld_authority_info.map(|acc| acc.key),
            transfer_fee_basis_points,
            maximum_fee,
        )?,
        &[token_program.clone(), mint_info.clone()],
    )?;

    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn withdraw_withheld_tokens<'a, 'info>(
    mint_info: &'a AccountInfo<'info>,
    destination_info: &'a AccountInfo<'info>,
    authority_info: &'a AccountInfo<'info>,
    signer_info: &AccountInfo<'info>,
    token_program: &'a AccountInfo<'info>,
    source_infos: &[&AccountInfo<'info>],
) -> ProgramResult {
    let sources: Vec<&Pubkey> = source_infos.iter().map(|signer| signer.key).collect();
    let mut account_infos = vec![
        token_program.clone(),
        mint_info.clone(),
        destination_info.clone(),
        authority_info.clone(),
        signer_info.clone(),
    ];
    account_infos.extend(source_infos.iter().map(|s| (*s).clone()));

    solana_program::program::invoke(
        &spl_token_2022::extension::transfer_fee::instruction::withdraw_withheld_tokens_from_accounts(
            &spl_token_2022::id(),
            mint_info.key,
            destination_info.key,
            authority_info.key,
            &[signer_info.key],
            &sources
        )?,
        &account_infos
    )?;

    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn withdraw_withheld_tokens_from_mint<'a, 'info>(
    mint_info: &'a AccountInfo<'info>,
    destination_info: &'a AccountInfo<'info>,
    authority_info: &'a AccountInfo<'info>,
    signer_info: &'a AccountInfo<'info>,
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::transfer_fee::instruction::withdraw_withheld_tokens_from_mint(
            &spl_token_2022::id(),
            mint_info.key,
            destination_info.key,
            authority_info.key,
            &[signer_info.key],
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
            destination_info.clone(),
            authority_info.clone(),
            signer_info.clone(),
        ],
    )?;

    Ok(())
}

// TRANSFER HOOK EXTENSION
#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn update_transfer_hook<'a, 'info>(
    mint_info: &'a AccountInfo<'info>, // the mint account
    authority_info: &'a AccountInfo<'info>,
    signer_info: &'a AccountInfo<'info>,
    transfer_hook_program_info: Option<&'a AccountInfo<'info>>,
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::transfer_hook::instruction::update(
            &spl_token_2022::id(),
            mint_info.key,
            authority_info.key,
            &[signer_info.key],
            transfer_hook_program_info.map(|acc| *acc.key),
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
            authority_info.clone(),
            signer_info.clone(),
        ],
    )?;
    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn initialize_transfer_hook<'a, 'info>(
    mint_info: &'a AccountInfo<'info>, // the mint account
    authority_info: Option<&'a AccountInfo<'info>>,
    transfer_hook_program_info: Option<&'a AccountInfo<'info>>,
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::transfer_hook::instruction::initialize(
            &spl_token_2022::id(),
            mint_info.key,
            authority_info.map(|acc| *acc.key),
            transfer_hook_program_info.map(|acc| *acc.key),
        )?,
        &[token_program.clone(), mint_info.clone()],
    )?;
    Ok(())
}

//METADATA POINTER
#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn initialize_metadata_pointer<'a, 'info>(
    mint_info: &'a AccountInfo<'info>, // the mint account
    authority_info: Option<&'a AccountInfo<'info>>,
    metadata_address_info: Option<&'a AccountInfo<'info>>,
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::metadata_pointer::instruction::initialize(
            &spl_token_2022::id(),
            mint_info.key,
            authority_info.map(|acc| *acc.key),
            metadata_address_info.map(|acc| *acc.key),
        )?,
        &[token_program.clone(), mint_info.clone()],
    )?;
    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn update_metadata_pointer<'info>(
    mint_info: &AccountInfo<'info>, // the mint account
    authority_info: &AccountInfo<'info>,
    signer_info: &AccountInfo<'info>,
    metadata_address_info: Option<&AccountInfo<'info>>,
    token_program: &AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::metadata_pointer::instruction::update(
            &spl_token_2022::id(),
            mint_info.key,
            authority_info.key,
            &[signer_info.key],
            metadata_address_info.map(|acc| *acc.key),
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
            authority_info.clone(),
            signer_info.clone(),
        ],
    )?;
    Ok(())
}
//Pausable extension is not exported from spl_token_2022

//MEMO TRANSFER
#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn enable_required_transfer_memos<'a, 'info>(
    account_to_update_info: &'a AccountInfo<'info>, // The account to update
    owner_info: &'a AccountInfo<'info>,             // The owner account
    signer_info: &'a AccountInfo<'info>,
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::memo_transfer::instruction::enable_required_transfer_memos(
            &spl_token_2022::id(),
            account_to_update_info.key,
            owner_info.key,
            &[signer_info.key],
        )?,
        &[
            account_to_update_info.clone(),
            owner_info.clone(),
            signer_info.clone(),
            token_program.clone(),
        ],
    )?;
    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn disable_required_transfer_memos<'a, 'info>(
    account_to_update_info: &'a AccountInfo<'info>, // The account to update
    owner_info: &'a AccountInfo<'info>,             // The owner account
    signer_info: &'a AccountInfo<'info>,
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::memo_transfer::instruction::disable_required_transfer_memos(
            &spl_token_2022::id(),
            account_to_update_info.key,
            owner_info.key,
            &[signer_info.key],
        )?,
        &[
            account_to_update_info.clone(),
            owner_info.clone(),
            signer_info.clone(),
            token_program.clone(),
        ],
    )?;
    Ok(())
}

//INTEREST BEARING MINT EXTENSION
#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn initialize_interest_bearing_mint<'a, 'info>(
    mint_info: &'a AccountInfo<'info>, // The mint account
    rate_authority_info: Option<&'a AccountInfo<'info>>, // Optional rate authority
    token_program: &'a AccountInfo<'info>,
    rate: i16, // Initial interest rate
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::interest_bearing_mint::instruction::initialize(
            &spl_token_2022::id(),
            mint_info.key,
            rate_authority_info.map(|acc| *acc.key),
            rate,
        )?,
        &[token_program.clone(), mint_info.clone()],
    )?;
    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn update_interest_rate<'a, 'info>(
    mint_info: &'a AccountInfo<'info>, // The mint account
    rate_authority_info: &'a AccountInfo<'info>,
    signer_info: &'a AccountInfo<'info>,
    token_program: &'a AccountInfo<'info>,
    rate: i16, // New interest rate
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::interest_bearing_mint::instruction::update_rate(
            &spl_token_2022::id(),
            mint_info.key,
            rate_authority_info.key,
            &[signer_info.key],
            rate,
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
            rate_authority_info.clone(),
            signer_info.clone(),
        ],
    )?;
    Ok(())
}

//GROUP POINTER EXTENSION

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn initialize_group_pointer<'a, 'info>(
    mint_info: &'a AccountInfo<'info>, // the mint account
    authority_info: Option<&'a AccountInfo<'info>>,
    group_address_info: Option<&'a AccountInfo<'info>>,
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::group_pointer::instruction::initialize(
            &spl_token_2022::id(),
            mint_info.key,
            authority_info.map(|acc| *acc.key),
            group_address_info.map(|acc| *acc.key),
        )?,
        &[token_program.clone(), mint_info.clone()],
    )?;
    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn update_group_pointer<'a, 'info>(
    mint_info: &'a AccountInfo<'info>, // the mint account
    authority_info: &'a AccountInfo<'info>,
    signer_info: &'a AccountInfo<'info>,
    group_address_info: Option<&'a AccountInfo<'info>>,
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::group_pointer::instruction::update(
            &spl_token_2022::id(),
            mint_info.key,
            authority_info.key,
            &[signer_info.key],
            group_address_info.map(|acc| *acc.key),
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
            authority_info.clone(),
            signer_info.clone(),
        ],
    )?;
    Ok(())
}

//GROUP MEMBER POINTER

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn initialize_group_member_pointer<'a, 'info>(
    mint_info: &'a AccountInfo<'info>, // The mint account
    authority_info: Option<&'a AccountInfo<'info>>,
    member_address_info: Option<&'a AccountInfo<'info>>,
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::group_member_pointer::instruction::initialize(
            &spl_token_2022::id(),
            mint_info.key,
            authority_info.map(|acc| *acc.key),
            member_address_info.map(|acc| *acc.key),
        )?,
        &[mint_info.clone(), token_program.clone()],
    )?;
    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn update_group_member_pointer<'a, 'info>(
    mint_info: &'a AccountInfo<'info>, // The mint account
    authority_info: &'a AccountInfo<'info>,
    signer_info: &'a AccountInfo<'info>, // Multiple signers for multisig support
    member_address_info: Option<&'a AccountInfo<'info>>,
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::group_member_pointer::instruction::update(
            &spl_token_2022::id(),
            mint_info.key,
            authority_info.key,
            &[signer_info.key],
            member_address_info.map(|acc| *acc.key),
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
            authority_info.clone(),
        ],
    )?;
    Ok(())
}

//ACCOUNT STATE
#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn initialize_default_account_state<'a, 'info>(
    mint_info: &'a AccountInfo<'info>, // The mint account
    token_program: &'a AccountInfo<'info>,
    state: &AccountState,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::default_account_state::instruction::initialize_default_account_state(
            &spl_token_2022::id(),
            mint_info.key,
            state,
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
        ],
    )?;
    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn update_default_account_state<'a, 'info>(
    mint_info: &'a AccountInfo<'info>, // The mint account
    freeze_authority_info: &'a AccountInfo<'info>,
    signer_info: &'a AccountInfo<'info>,
    token_program: &'a AccountInfo<'info>,
    state: &AccountState,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::default_account_state::instruction::update_default_account_state(
            &spl_token_2022::id(),
            mint_info.key,
            freeze_authority_info.key,
            &[signer_info.key],
            state,
        )?,
      &[
        token_program.clone(),
        mint_info.clone(),
        freeze_authority_info.clone(),
        signer_info.clone()
    ]
    )?;
    Ok(())
}

//CPI GUARD
#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn enable_cpi_guard<'a, 'info>(
    account_info: &'a AccountInfo<'info>, // the account to update
    owner_info: &'a AccountInfo<'info>,   // the account owner
    signer_info: &'a AccountInfo<'info>,  // optional multisig signers
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::cpi_guard::instruction::enable_cpi_guard(
            &spl_token_2022::id(),
            account_info.key,
            owner_info.key,
            &[signer_info.key],
        )?,
        &[
            token_program.clone(),
            account_info.clone(),
            owner_info.clone(),
            signer_info.clone(),
        ],
    )?;
    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn disable_cpi_guard<'a, 'info>(
    account_info: &'a AccountInfo<'info>, // the account to update
    owner_info: &'a AccountInfo<'info>,   // the account owner
    signer_info: &'a AccountInfo<'info>,  // optional multisig signers
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::cpi_guard::instruction::disable_cpi_guard(
            &spl_token_2022::id(),
            account_info.key,
            owner_info.key,
            &[signer_info.key],
        )?,
        &[
            token_program.clone(),
            account_info.clone(),
            owner_info.clone(),
            signer_info.clone(),
        ],
    )?;
    Ok(())
}

//CONFIDENTIAL_TRANSFER
// transfer_confidential
// transfer_with_fee_confidential

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn initialize_confidential_mint<'a, 'info>(
    mint: &'a AccountInfo<'info>,
    authority: Option<&'a AccountInfo<'info>>,
    auto_approve_new_accounts: bool,
    auditor_elgamal_pubkey: Option<ElGamalPubkey>,
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::confidential_transfer::instruction::initialize_mint(
            &spl_token_2022::id(),
            mint.key,
            authority.map(|i| *i.key),
            auto_approve_new_accounts,
            auditor_elgamal_pubkey,
        )?,
        &[token_program.clone(), mint.clone()],
    )?;
    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn update_confidential_mint<'a, 'info>(
    mint_info: &'a AccountInfo<'info>,
    authority_info: Option<&AccountInfo<'info>>,
    multi_signer_infos: &[&'a AccountInfo<'info>],
    auto_approve_new_accounts: bool,
    auditor_elgamal_pubkey: Option<ElGamalPubkey>,
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    let multi_signers: Vec<&Pubkey> = multi_signer_infos.iter().map(|i| i.key).collect();

    let mut accounts = vec![token_program.clone(), mint_info.clone()];

    accounts.extend(multi_signer_infos.iter().map(|a| (*a).clone()));

    solana_program::program::invoke(
        &spl_token_2022::extension::confidential_transfer::instruction::update_mint(
            &spl_token_2022::id(),
            mint_info.key,
            authority_info.map(|i| i.key).unwrap(), // TODO
            &multi_signers,
            auto_approve_new_accounts,
            auditor_elgamal_pubkey,
        )?,
        &accounts,
    )?;
    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn confidential_deposit<'a, 'info>(
    token_account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    amount: u64,
    decimals: u8,
    authority_info: &AccountInfo<'info>,
    multi_signer_infos: &[&'a AccountInfo<'info>],
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    let multi_signers: Vec<&Pubkey> = multi_signer_infos.iter().map(|i| i.key).collect();

    let mut accounts = vec![
        authority_info.clone(),
        token_program.clone(),
        mint_info.clone(),
    ];

    accounts.extend(multi_signer_infos.iter().map(|a| (*a).clone()));

    solana_program::program::invoke(
        &spl_token_2022::extension::confidential_transfer::instruction::deposit(
            &spl_token_2022::id(),
            token_account_info.key,
            mint_info.key,
            amount,
            decimals,
            authority_info.key,
            &multi_signers,
        )?,
        &accounts,
    )?;
    Ok(())
}


#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn enable_confidential_deposit<'a, 'info>(
    token_account_info: &AccountInfo<'info>,
    mint_info: &AccountInfo<'info>,
    amount: u64,
    decimals: u8,
    authority_info: &AccountInfo<'info>,
    multi_signer_infos: &[&'a AccountInfo<'info>],
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    let multi_signers: Vec<&Pubkey> = multi_signer_infos.iter().map(|i| i.key).collect();

    let mut accounts = vec![
        authority_info.clone(),
        token_program.clone(),
        mint_info.clone(),
    ];

    accounts.extend(multi_signer_infos.iter().map(|a| (*a).clone()));

    solana_program::program::invoke(
        &spl_token_2022::extension::confidential_transfer::instruction::deposit(
            &spl_token_2022::id(),
            token_account_info.key,
            mint_info.key,
            amount,
            decimals,
            authority_info.key,
            &multi_signers,
        )?,
        &accounts,
    )?;
    Ok(())
}


//CONFIDENTIAL TRANSFER FEE
// withdraw_withheld_tokens_from_mint_confidential
// withdraw_withheld_tokens_from_accounts_confidential

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn initialize_confidential_transfer_fee_config<'a, 'info>(
    mint_info: &'a AccountInfo<'info>,           // The mint account
    authority_info: Option<&AccountInfo<'info>>, // The confidential transfer fee authority
    withdraw_withheld_authority_elgamal_pubkey: ElGamalPubkey, // The ElGamal public key
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    solana_program::program::invoke(
        &spl_token_2022::extension::confidential_transfer_fee::instruction::initialize_confidential_transfer_fee_config(
            &spl_token_2022::id(),
            mint_info.key,
            authority_info.map(|x| *x.key),
            withdraw_withheld_authority_elgamal_pubkey,
        )?,
        &[
            token_program.clone(),
            mint_info.clone(),
        ]
    )?;
    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn harvest_withheld_tokens_to_mint_confidential<'a, 'info>(
    mint_info: &'a AccountInfo<'info>,              // The mint account
    source_accounts_info: &'a [AccountInfo<'info>], // List of source accounts to harvest from
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    let sources = source_accounts_info
        .iter()
        .map(|x| x.key)
        .collect::<Vec<_>>();

    let mut account_infos = vec![token_program.clone(), mint_info.clone()];

    account_infos.extend(source_accounts_info.iter().map(|s| (*s).clone()));

    solana_program::program::invoke(
        &spl_token_2022::extension::confidential_transfer_fee::instruction::harvest_withheld_tokens_to_mint(
            &spl_token_2022::id(),
            mint_info.key,
            &sources
        )?,
        &account_infos
    )?;
    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn enable_harvest_to_mint_confidential<'a, 'info>(
    mint_info: &'a AccountInfo<'info>,          // The mint account
    fee_authority_info: &'a AccountInfo<'info>, // The confidential transfer fee authority
    multi_signer_infos: &[&'a AccountInfo<'info>],
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    let multi_signers: Vec<&Pubkey> = multi_signer_infos.iter().map(|i| i.key).collect();

    let mut accounts = vec![
        token_program.clone(),
        mint_info.clone(),
        fee_authority_info.clone(),
    ];

    accounts.extend(multi_signer_infos.iter().map(|a| (*a).clone()));

    solana_program::program::invoke(
        &spl_token_2022::extension::confidential_transfer_fee::instruction::enable_harvest_to_mint(
            &spl_token_2022::id(),
            mint_info.key,
            fee_authority_info.key,
            &multi_signers,
        )?,
        &accounts,
    )?;
    Ok(())
}

#[cfg(feature = "spl-2022")]
#[inline(always)]
pub fn disable_harvest_to_mint_confidential<'a, 'info>(
    mint_info: &'a AccountInfo<'info>,          // The mint account
    fee_authority_info: &'a AccountInfo<'info>, // The confidential transfer fee authority
    multi_signer_infos: &[&'a AccountInfo<'info>],
    token_program: &'a AccountInfo<'info>,
) -> ProgramResult {
    let multi_signers: Vec<&Pubkey> = multi_signer_infos.iter().map(|i| i.key).collect();

    let mut accounts = vec![
        token_program.clone(),
        mint_info.clone(),
        fee_authority_info.clone(),
    ];

    accounts.extend(multi_signer_infos.iter().map(|a| (*a).clone()));

    solana_program::program::invoke(
        &spl_token_2022::extension::confidential_transfer_fee::instruction::disable_harvest_to_mint(
            &spl_token_2022::id(),
            mint_info.key,
            fee_authority_info.key,
            &multi_signers
        )?,
        &accounts
    )?;
    Ok(())
}
