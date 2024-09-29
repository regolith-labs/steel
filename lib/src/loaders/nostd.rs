#![cfg(feature = "nostd")]

use bytemuck::Pod;
use solana_nostd_entrypoint::NoStdAccountInfo;
#[cfg(feature = "spl")]
use solana_program::program_pack::Pack;
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

use crate::{AccountDeserialize, AccountInfoValidation, Discriminator, ToAccount};

impl AccountInfoValidation for NoStdAccountInfo {
    fn is_signer(&self) -> Result<&Self, ProgramError> {
        if !self.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(self)
    }

    fn is_writable(&self) -> Result<&Self, ProgramError> {
        if !self.is_writable() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(self)
    }

    fn is_executable(&self) -> Result<&Self, ProgramError> {
        if !self.executable() {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(self)
    }

    fn is_empty(&self) -> Result<&Self, ProgramError> {
        let account_data = self
            .try_borrow_data()
            .ok_or(ProgramError::UninitializedAccount)?;
        if account_data.is_empty() {
            return Err(ProgramError::UninitializedAccount);
        }
        Ok(self)
    }

    fn is_program(&self, program_id: &Pubkey) -> Result<&Self, ProgramError> {
        self.has_address(program_id)?.is_executable()
    }

    fn is_type<T: Discriminator>(&self, program_id: &Pubkey) -> Result<&Self, ProgramError> {
        self.has_owner(program_id)?;
        if self
            .try_borrow_data()
            .ok_or(ProgramError::AccountBorrowFailed)?[0]
            .ne(&T::discriminator())
        {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(self)
    }

    fn has_owner(&self, owner: &Pubkey) -> Result<&Self, ProgramError> {
        if self.owner().ne(owner) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(self)
    }

    fn has_address(&self, address: &Pubkey) -> Result<&Self, ProgramError> {
        if self.key().ne(&address) {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(self)
    }

    fn has_seeds(
        &self,
        seeds: &[&[u8]],
        bump: u8,
        program_id: &Pubkey,
    ) -> Result<&Self, ProgramError> {
        let pda = Pubkey::find_program_address(seeds, program_id);
        if self.key().ne(&pda.0) || bump.ne(&pda.1) {
            return Err(ProgramError::InvalidSeeds);
        }
        Ok(self)
    }

    fn is_sysvar(&self, sysvar_id: &Pubkey) -> Result<&Self, ProgramError> {
        self.has_owner(&solana_program::sysvar::ID)?
            .has_address(sysvar_id)
    }
}

impl ToAccount for NoStdAccountInfo {
    fn to_account<T: AccountDeserialize + Discriminator + Pod>(
        &self,
        program_id: &Pubkey,
    ) -> Result<&T, ProgramError> {
        unsafe {
            self.has_owner(program_id)?;
            T::try_from_bytes(std::slice::from_raw_parts(
                self.try_borrow_data()
                    .ok_or(ProgramError::AccountBorrowFailed)?
                    .as_ptr(),
                8 + std::mem::size_of::<T>(),
            ))
        }
    }

    fn to_account_mut<T: AccountDeserialize + Discriminator + Pod>(
        &self,
        program_id: &Pubkey,
    ) -> Result<&mut T, ProgramError> {
        if !self.is_writable() {
            return Err(ProgramError::AccountBorrowFailed);
        }
        unsafe {
            self.has_owner(program_id)?;
            T::try_from_bytes_mut(std::slice::from_raw_parts_mut(
                self.try_borrow_mut_data()
                    .ok_or(ProgramError::AccountBorrowFailed)?
                    .as_mut_ptr(),
                8 + std::mem::size_of::<T>(),
            ))
        }
    }
}

#[cfg(feature = "spl")]
impl crate::ToSplToken for NoStdAccountInfo {
    fn to_mint(&self) -> Result<spl_token::state::Mint, ProgramError> {
        unsafe {
            self.has_owner(&spl_token::ID)?;
            spl_token::state::Mint::unpack(std::slice::from_raw_parts(
                self.try_borrow_data()
                    .ok_or(ProgramError::AccountBorrowFailed)?
                    .as_ptr(),
                spl_token::state::Mint::LEN,
            ))
        }
    }
    fn to_token_account(&self) -> Result<spl_token::state::Account, ProgramError> {
        unsafe {
            self.has_owner(&spl_token::ID)?;
            spl_token::state::Account::unpack(std::slice::from_raw_parts(
                self.try_borrow_data()
                    .ok_or(ProgramError::AccountBorrowFailed)?
                    .as_ptr(),
                spl_token::state::Account::LEN,
            ))
        }
    }
    fn to_associated_token_account(
        &self,
        owner: &Pubkey,
        mint: &Pubkey,
    ) -> Result<spl_token::state::Account, ProgramError> {
        self.has_address(&spl_associated_token_account::get_associated_token_address(
            owner, mint,
        ))?
        .to_token_account()
    }
}
