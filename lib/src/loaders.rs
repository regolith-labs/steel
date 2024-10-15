#[cfg(feature = "spl")]
use solana_program::program_pack::Pack;
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    AccountDeserialize, AccountDeserializeMut, AccountInfoValidation, AsAccount, Discriminator,
    FromHeader, FromHeaderMut,
};
#[cfg(feature = "spl")]
use crate::{AccountValidation, AsSplToken};

impl AccountInfoValidation for AccountInfo<'_> {
    fn is_signer(&self) -> Result<&Self, ProgramError> {
        if !self.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(self)
    }

    fn is_writable(&self) -> Result<&Self, ProgramError> {
        if !self.is_writable {
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(self)
    }

    fn is_executable(&self) -> Result<&Self, ProgramError> {
        if !self.executable {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(self)
    }

    fn is_empty(&self) -> Result<&Self, ProgramError> {
        if !self.data_is_empty() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        Ok(self)
    }

    fn is_program(&self, program_id: &Pubkey) -> Result<&Self, ProgramError> {
        self.has_address(program_id)?.is_executable()
    }

    fn is_type<T: Discriminator>(&self, program_id: &Pubkey) -> Result<&Self, ProgramError> {
        self.has_owner(program_id)?;
        if self.try_borrow_data()?[0].ne(&T::discriminator()) {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(self)
    }

    fn has_owner(&self, owner: &Pubkey) -> Result<&Self, ProgramError> {
        if self.owner.ne(owner) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(self)
    }

    fn has_address(&self, address: &Pubkey) -> Result<&Self, ProgramError> {
        if self.key.ne(&address) {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(self)
    }

    fn has_seeds(&self, seeds: &[&[u8]], program_id: &Pubkey) -> Result<&Self, ProgramError> {
        let pda = Pubkey::find_program_address(seeds, program_id);
        if self.key.ne(&pda.0) {
            return Err(ProgramError::InvalidSeeds);
        }
        Ok(self)
    }

    fn is_sysvar(&self, sysvar_id: &Pubkey) -> Result<&Self, ProgramError> {
        self.has_owner(&solana_program::sysvar::ID)?
            .has_address(sysvar_id)
    }
}

impl<'a> AsAccount<'a> for AccountInfo<'a> {
    fn as_account<T: AccountDeserialize<'a>>(
        &'a self,
        program_id: &Pubkey,
    ) -> Result<&T, ProgramError> {
        self.has_owner(program_id)?;
        unsafe {
            T::try_from_bytes(std::slice::from_raw_parts(
                self.try_borrow_data()?.as_ptr(),
                8 + std::mem::size_of::<T>(),
            ))
        }
    }

    fn as_account_with_header<H, T>(&'a self, program_id: &Pubkey) -> Result<T, ProgramError>
    where
        H: AccountDeserialize<'a>,
        T: FromHeader<'a, H>,
    {
        self.has_owner(program_id)?;
        unsafe {
            let borrowed = self.try_borrow_data()?;
            let len = borrowed.len();
            let data = std::slice::from_raw_parts(borrowed.as_ptr(), len);
            let (header_bytes, remainder) = data.split_at(8 + std::mem::size_of::<H>());
            let header = AccountDeserialize::try_from_bytes(header_bytes)?;
            T::from_header_and_remainder(header, remainder)
        }
    }

    fn as_account_mut<T: AccountDeserializeMut<'a>>(
        &'a self,
        program_id: &Pubkey,
    ) -> Result<&mut T, ProgramError> {
        self.has_owner(program_id)?;
        unsafe {
            T::try_from_bytes_mut(std::slice::from_raw_parts_mut(
                self.try_borrow_mut_data()?.as_mut_ptr(),
                8 + std::mem::size_of::<T>(),
            ))
        }
    }

    fn as_account_mut_with_header<H, T>(&'a self, program_id: &Pubkey) -> Result<T, ProgramError>
    where
        H: AccountDeserializeMut<'a>,
        T: FromHeaderMut<'a, H>,
    {
        self.has_owner(program_id)?;
        unsafe {
            let mut borrowed = self.try_borrow_mut_data()?;
            let len = borrowed.len();
            let data = std::slice::from_raw_parts_mut(borrowed.as_mut_ptr(), len);
            let (header_bytes, remainder) = data.split_at_mut(8 + std::mem::size_of::<H>());
            let header: &mut H = AccountDeserializeMut::try_from_bytes_mut(header_bytes)?;
            T::from_header_and_remainder_mut(header, remainder)
        }
    }
}

#[cfg(feature = "spl")]
impl AsSplToken for AccountInfo<'_> {
    fn as_mint(&self) -> Result<spl_token::state::Mint, ProgramError> {
        unsafe {
            self.has_owner(&spl_token::ID)?;
            spl_token::state::Mint::unpack(std::slice::from_raw_parts(
                self.try_borrow_data()?.as_ptr(),
                spl_token::state::Mint::LEN,
            ))
        }
    }
    fn as_token_account(&self) -> Result<spl_token::state::Account, ProgramError> {
        unsafe {
            self.has_owner(&spl_token::ID)?;
            spl_token::state::Account::unpack(std::slice::from_raw_parts(
                self.try_borrow_data()?.as_ptr(),
                spl_token::state::Account::LEN,
            ))
        }
    }
    fn as_associated_token_account(
        &self,
        owner: &Pubkey,
        mint: &Pubkey,
    ) -> Result<spl_token::state::Account, ProgramError> {
        self.has_address(&spl_associated_token_account::get_associated_token_address(
            owner, mint,
        ))?
        .as_token_account()
    }
}

#[cfg(feature = "spl")]
impl AccountValidation for spl_token::state::Mint {
    fn assert<F>(&self, condition: F) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        if !condition(self) {
            return Err(solana_program::program_error::ProgramError::InvalidAccountData);
        }
        Ok(self)
    }

    fn assert_with_msg<F>(&self, condition: F, msg: &str) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        if let Err(err) = crate::assert_with_msg(
            condition(self),
            solana_program::program_error::ProgramError::InvalidAccountData,
            msg,
        ) {
            return Err(err.into());
        }
        Ok(self)
    }

    fn assert_mut<F>(&mut self, _condition: F) -> Result<&mut Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        panic!("not implemented")
    }

    fn assert_mut_with_msg<F>(
        &mut self,
        _condition: F,
        _msg: &str,
    ) -> Result<&mut Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        panic!("not implemented")
    }
}

#[cfg(feature = "spl")]
impl AccountValidation for spl_token::state::Account {
    fn assert<F>(&self, condition: F) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        if !condition(self) {
            return Err(solana_program::program_error::ProgramError::InvalidAccountData);
        }
        Ok(self)
    }

    fn assert_with_msg<F>(&self, condition: F, msg: &str) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        if let Err(err) = crate::assert_with_msg(
            condition(self),
            solana_program::program_error::ProgramError::InvalidAccountData,
            msg,
        ) {
            return Err(err.into());
        }
        Ok(self)
    }

    fn assert_mut<F>(&mut self, _condition: F) -> Result<&mut Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        panic!("not implemented")
    }

    fn assert_mut_with_msg<F>(
        &mut self,
        _condition: F,
        _msg: &str,
    ) -> Result<&mut Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        panic!("not implemented")
    }
}
