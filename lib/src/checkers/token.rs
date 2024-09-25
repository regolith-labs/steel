#[allow(dead_code)]

use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::state::Mint;
use spl_token_2022::check_spl_token_program_account;
use std::ops::Deref;

use crate::assert_with_msg;

#[derive(Clone)]
pub struct MintAccountInfo<'a, 'info> {
    pub mint: Mint,
    pub info: &'a AccountInfo<'info>,
}

impl<'a, 'info> MintAccountInfo<'a, 'info> {
    #[allow(dead_code)]
    pub fn new(info: &'a AccountInfo<'info>) -> Result<MintAccountInfo<'a, 'info>, ProgramError> {
        check_spl_token_program_account(info.owner)?;
        let mint: Mint = Mint::unpack(&info.try_borrow_data()?)?;

        Ok(Self { mint, info })
    }
}

impl<'a, 'info> AsRef<AccountInfo<'info>> for MintAccountInfo<'a, 'info> {
    fn as_ref(&self) -> &AccountInfo<'info> {
        self.info
    }
}

#[derive(Clone)]
pub struct TokenAccountInfo<'a, 'info> {
    pub info: &'a AccountInfo<'info>,
}

impl<'a, 'info> TokenAccountInfo<'a, 'info> {
    #[allow(dead_code)]
    pub fn new(
        info: &'a AccountInfo<'info>,
        mint: &Pubkey,
    ) -> Result<TokenAccountInfo<'a, 'info>, ProgramError> {
        assert_with_msg(
            info.owner == &spl_token::id() || info.owner == &spl_token_2022::id(),
            ProgramError::IllegalOwner,
            "Token account must be owned by the Token Program",
        )?;
        // The mint key is found at offset 0 of the token account
        assert_with_msg(
            &info.try_borrow_data()?[0..32] == mint.as_ref(),
            ProgramError::InvalidAccountData,
            "Token account mint mismatch",
        )?;
        Ok(Self { info })
    }

    #[allow(dead_code)]
    pub fn get_owner(&self) -> Pubkey {
        Pubkey::new_from_array(
            self.info.try_borrow_data().unwrap()[32..64]
                .try_into()
                .unwrap(),
        )
    }

    #[allow(dead_code)]
    pub fn new_with_owner(
        info: &'a AccountInfo<'info>,
        mint: &Pubkey,
        owner: &Pubkey,
    ) -> Result<TokenAccountInfo<'a, 'info>, ProgramError> {
        let token_account_info = Self::new(info, mint)?;
        // The owner key is found at offset 32 of the token account
        assert_with_msg(
            &info.try_borrow_data()?[32..64] == owner.as_ref(),
            ProgramError::IllegalOwner,
            "Token account owner mismatch",
        )?;
        Ok(token_account_info)
    }

    #[allow(dead_code)]
    pub fn new_with_owner_and_key(
        info: &'a AccountInfo<'info>,
        mint: &Pubkey,
        owner: &Pubkey,
        key: &Pubkey,
    ) -> Result<TokenAccountInfo<'a, 'info>, ProgramError> {
        assert_with_msg(
            info.key == key,
            ProgramError::InvalidInstructionData,
            "Invalid pubkey for Token Account",
        )?;
        Self::new_with_owner(info, mint, owner)
    }
}

impl<'a, 'info> AsRef<AccountInfo<'info>> for TokenAccountInfo<'a, 'info> {
    fn as_ref(&self) -> &AccountInfo<'info> {
        self.info
    }
}

#[allow(dead_code)]
impl<'a, 'info> Deref for TokenAccountInfo<'a, 'info> {
    type Target = AccountInfo<'info>;

    fn deref(&self) -> &Self::Target {
        self.info
    }
}
