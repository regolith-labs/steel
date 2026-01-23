use pinocchio::Address;

pub enum Mint {
    V0(pinocchio_token::state::Mint),
    V1(pinocchio_token_2022::state::Mint),
}

impl Mint {
    pub fn mint_authority(&self) -> Option<&Address> {
        match self {
            Mint::V0(mint) => mint.mint_authority(),
            Mint::V1(mint) => mint.mint_authority(),
        }
    }

    pub fn supply(&self) -> u64 {
        match self {
            Mint::V0(mint) => mint.supply(),
            Mint::V1(mint) => mint.supply(),
        }
    }

    pub fn decimals(&self) -> u8 {
        match self {
            Mint::V0(mint) => mint.decimals(),
            Mint::V1(mint) => mint.decimals(),
        }
    }

    pub fn is_initialized(&self) -> bool {
        match self {
            Mint::V0(mint) => mint.is_initialized(),
            Mint::V1(mint) => mint.is_initialized(),
        }
    }

    pub fn freeze_authority(&self) -> Option<&Address> {
        match self {
            Mint::V0(mint) => mint.freeze_authority(),
            Mint::V1(mint) => mint.freeze_authority(),
        }
    }
}
