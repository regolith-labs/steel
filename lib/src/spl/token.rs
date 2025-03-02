use solana_program::{program_option::COption, pubkey::Pubkey};

pub enum TokenAccount {
    V0(spl_token::state::Account),
    V1(spl_token_2022::state::Account),
}

impl TokenAccount {
    pub fn mint(&self) -> Pubkey {
        match self {
            TokenAccount::V0(account) => account.mint,
            TokenAccount::V1(account) => account.mint,
        }
    }

    pub fn owner(&self) -> Pubkey {
        match self {
            TokenAccount::V0(account) => account.owner,
            TokenAccount::V1(account) => account.owner,
        }
    }

    pub fn amount(&self) -> u64 {
        match self {
            TokenAccount::V0(account) => account.amount,
            TokenAccount::V1(account) => account.amount,
        }
    }

    pub fn delegate(&self) -> COption<Pubkey> {
        match self {
            TokenAccount::V0(account) => account.delegate,
            TokenAccount::V1(account) => account.delegate,
        }
    }

    pub fn is_frozen(&self) -> bool {
        match self {
            TokenAccount::V0(account) => account.is_frozen(),
            TokenAccount::V1(account) => account.is_frozen(),
        }
    }

    pub fn is_native(&self) -> COption<u64> {
        match self {
            TokenAccount::V0(account) => account.is_native,
            TokenAccount::V1(account) => account.is_native,
        }
    }

    pub fn delegated_amount(&self) -> u64 {
        match self {
            TokenAccount::V0(account) => account.delegated_amount,
            TokenAccount::V1(account) => account.delegated_amount,
        }
    }

    pub fn close_authority(&self) -> COption<Pubkey> {
        match self {
            TokenAccount::V0(account) => account.close_authority,
            TokenAccount::V1(account) => account.close_authority,
        }
    }
}
