use pinocchio::Address;

pub enum TokenAccount {
    V0(pinocchio_token::state::TokenAccount),
    V1(pinocchio_token_2022::state::TokenAccount),
}

impl TokenAccount {
    pub fn mint(&self) -> &Address {
        match self {
            TokenAccount::V0(account) => account.mint(),
            TokenAccount::V1(account) => account.mint(),
        }
    }

    pub fn owner(&self) -> &Address {
        match self {
            TokenAccount::V0(account) => account.owner(),
            TokenAccount::V1(account) => account.owner(),
        }
    }

    pub fn amount(&self) -> u64 {
        match self {
            TokenAccount::V0(account) => account.amount(),
            TokenAccount::V1(account) => account.amount(),
        }
    }

    pub fn delegate(&self) -> Option<&Address> {
        match self {
            TokenAccount::V0(account) => account.delegate(),
            TokenAccount::V1(account) => account.delegate(),
        }
    }

    pub fn is_frozen(&self) -> bool {
        match self {
            TokenAccount::V0(account) => account.is_frozen(),
            TokenAccount::V1(account) => account.is_frozen(),
        }
    }

    pub fn is_native(&self) -> bool {
        match self {
            TokenAccount::V0(account) => account.is_native(),
            TokenAccount::V1(account) => account.is_native(),
        }
    }

    pub fn native_amount(&self) -> Option<u64> {
        match self {
            TokenAccount::V0(account) => account.native_amount(),
            TokenAccount::V1(account) => account.native_amount(),
        }
    }

    pub fn delegated_amount(&self) -> u64 {
        match self {
            TokenAccount::V0(account) => account.delegated_amount(),
            TokenAccount::V1(account) => account.delegated_amount(),
        }
    }

    pub fn close_authority(&self) -> Option<&Address> {
        match self {
            TokenAccount::V0(account) => account.close_authority(),
            TokenAccount::V1(account) => account.close_authority(),
        }
    }
}
