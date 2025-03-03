use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::{account::AccountInfoValidation, trace};

use solana_program::program_pack::Pack;

use crate::account::AccountValidation;

use super::{mint::Mint, token::TokenAccount};

pub trait AsSpl {
    fn as_mint(&self) -> Result<Mint, ProgramError>;
    fn as_token_account(&self) -> Result<TokenAccount, ProgramError>;
    fn as_associated_token_account(
        &self,
        owner: &Pubkey,
        mint: &Pubkey,
    ) -> Result<TokenAccount, ProgramError>;
}

impl AsSpl for AccountInfo<'_> {
    #[track_caller]
    fn as_mint(&self) -> Result<Mint, ProgramError> {
        match *self.owner {
            spl_token::ID => unsafe {
                // Validate account data length.
                let data = self.try_borrow_data()?;
                if data.len() != spl_token::state::Mint::LEN {
                    return Err(trace(
                        "Mint data length is invalid",
                        ProgramError::InvalidAccountData,
                    ));
                }

                // Deserialize account data.
                let mint = spl_token::state::Mint::unpack(std::slice::from_raw_parts(
                    data.as_ptr(),
                    spl_token::state::Mint::LEN,
                ))?;
                Ok(Mint::V0(mint))
            },
            spl_token_2022::ID => unsafe {
                // Validate account data length.
                let data = self.try_borrow_data()?;
                if data.len() != spl_token_2022::state::Mint::LEN {
                    return Err(trace(
                        "Mint data length is invalid",
                        ProgramError::InvalidAccountData,
                    ));
                }

                // Deserialize account data.
                let mint = spl_token_2022::state::Mint::unpack(std::slice::from_raw_parts(
                    data.as_ptr(),
                    spl_token_2022::state::Mint::LEN,
                ))?;
                Ok(Mint::V1(mint))
            },
            _ => return Err(ProgramError::InvalidAccountOwner),
        }
    }

    #[track_caller]
    fn as_token_account(&self) -> Result<TokenAccount, ProgramError> {
        match *self.owner {
            spl_token::ID => unsafe {
                // Validate account data length.
                let data = self.try_borrow_data()?;
                if data.len() != spl_token::state::Account::LEN {
                    return Err(trace(
                        "Token account data length is invalid",
                        ProgramError::InvalidAccountData,
                    ));
                }

                // Deserialize account data.
                let account = spl_token::state::Account::unpack(std::slice::from_raw_parts(
                    data.as_ptr(),
                    spl_token::state::Account::LEN,
                ))?;
                Ok(TokenAccount::V0(account))
            },
            spl_token_2022::ID => unsafe {
                // Validate account data length.
                let data = self.try_borrow_data()?;
                if data.len() != spl_token_2022::state::Account::LEN {
                    return Err(trace(
                        "Token account data length is invalid",
                        ProgramError::InvalidAccountData,
                    ));
                }

                // Deserialize account data.
                let account = spl_token_2022::state::Account::unpack(std::slice::from_raw_parts(
                    data.as_ptr(),
                    spl_token_2022::state::Account::LEN,
                ))?;
                Ok(TokenAccount::V1(account))
            },
            _ => return Err(ProgramError::InvalidAccountOwner),
        }
    }

    #[track_caller]
    fn as_associated_token_account(
        &self,
        owner: &Pubkey,
        mint: &Pubkey,
    ) -> Result<TokenAccount, ProgramError> {
        self.has_address(
            &spl_associated_token_account::get_associated_token_address_with_program_id(
                owner, mint, self.owner,
            ),
        )?
        .as_token_account()
    }
}

impl AccountValidation for Mint {
    #[track_caller]
    fn assert<F>(&self, condition: F) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        if !condition(self) {
            return Err(trace(
                "Mint data is invalid",
                solana_program::program_error::ProgramError::InvalidAccountData,
            ));
        }
        Ok(self)
    }

    #[track_caller]
    fn assert_err<F>(
        &self,
        condition: F,
        err: solana_program::program_error::ProgramError,
    ) -> Result<&Self, solana_program::program_error::ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        if !condition(self) {
            return Err(trace("Mint data is invalid", err));
        }
        Ok(self)
    }

    #[track_caller]
    fn assert_msg<F>(&self, condition: F, msg: &str) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        if !condition(self) {
            return Err(trace(
                format!("Mint data is invalid: {}", msg).as_str(),
                solana_program::program_error::ProgramError::InvalidAccountData,
            ));
        }
        Ok(self)
    }

    fn assert_mut<F>(&mut self, _condition: F) -> Result<&mut Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        panic!("not implemented")
    }

    fn assert_mut_err<F>(
        &mut self,
        _condition: F,
        _err: solana_program::program_error::ProgramError,
    ) -> Result<&mut Self, solana_program::program_error::ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        panic!("not implemented")
    }

    fn assert_mut_msg<F>(&mut self, _condition: F, _msg: &str) -> Result<&mut Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        panic!("not implemented")
    }
}

impl AccountValidation for TokenAccount {
    #[track_caller]
    fn assert<F>(&self, condition: F) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        if !condition(self) {
            return Err(trace(
                "Token account data is invalid",
                solana_program::program_error::ProgramError::InvalidAccountData,
            ));
        }
        Ok(self)
    }

    #[track_caller]
    fn assert_err<F>(
        &self,
        condition: F,
        err: solana_program::program_error::ProgramError,
    ) -> Result<&Self, solana_program::program_error::ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        if !condition(self) {
            return Err(trace("Token account data is invalid", err));
        }
        Ok(self)
    }

    #[track_caller]
    fn assert_msg<F>(&self, condition: F, msg: &str) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        if !condition(self) {
            return Err(trace(
                format!("Token account data is invalid: {}", msg).as_str(),
                solana_program::program_error::ProgramError::InvalidAccountData,
            ));
        }
        Ok(self)
    }

    fn assert_mut<F>(&mut self, _condition: F) -> Result<&mut Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        panic!("not implemented")
    }

    fn assert_mut_err<F>(
        &mut self,
        _condition: F,
        _err: solana_program::program_error::ProgramError,
    ) -> Result<&mut Self, solana_program::program_error::ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        panic!("not implemented")
    }

    fn assert_mut_msg<F>(&mut self, _condition: F, _msg: &str) -> Result<&mut Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        panic!("not implemented")
    }
}
