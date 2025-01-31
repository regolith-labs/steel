use bytemuck::Pod;
#[cfg(feature = "spl")]
use solana_program::program_pack::Pack;
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    AccountDeserialize, AccountInfoValidation, AsAccount, CloseAccount, Discriminator,
    LamportTransfer,
};
#[cfg(feature = "spl")]
use crate::{AccountValidation, AsSplToken};

impl AccountInfoValidation for AccountInfo<'_> {
    fn is_signer(&self) -> Result<&Self, ProgramError> {
        if !self.is_signer { Err(ProgramError::MissingRequiredSignature) } else { Ok(self) }
    }

    fn is_writable(&self) -> Result<&Self, ProgramError> {
        if !self.is_writable { Err(ProgramError::MissingRequiredSignature) } else { Ok(self) }
    }

    fn is_executable(&self) -> Result<&Self, ProgramError> {
        if !self.executable { Err(ProgramError::InvalidAccountData) } else { Ok(self) }
    }

    fn is_empty(&self) -> Result<&Self, ProgramError> {
        if !self.data_is_empty() { Err(ProgramError::AccountAlreadyInitialized) } else { Ok(self) }
    }

    fn is_program(&self, program_id: &Pubkey) -> Result<&Self, ProgramError> {
        self.has_address(program_id)?.is_executable()
    }

    fn is_type<T: Discriminator>(&self, program_id: &Pubkey) -> Result<&Self, ProgramError> {
        self.has_owner(program_id)?;
        if self.try_borrow_data()?[0].ne(&T::discriminator()) { Err(ProgramError::InvalidAccountData) } else { Ok(self) }
    }

    fn has_owner(&self, owner: &Pubkey) -> Result<&Self, ProgramError> {
        if self.owner.ne(owner) { Err(ProgramError::InvalidAccountOwner) } else { Ok(self) }
    }

    fn has_address(&self, address: &Pubkey) -> Result<&Self, ProgramError> {
        if self.key.ne(&address) { Err(ProgramError::InvalidAccountData) } else { Ok(self) }
    }

    fn has_seeds(&self, seeds: &[&[u8]], program_id: &Pubkey) -> Result<&Self, ProgramError> {
        let pda = Pubkey::find_program_address(seeds, program_id);
        if self.key.ne(&pda.0) { Err(ProgramError::InvalidSeeds) } else { Ok(self) }
    }

    fn is_sysvar(&self, sysvar_id: &Pubkey) -> Result<&Self, ProgramError> {
        self.has_owner(&solana_program::sysvar::ID)?
            .has_address(sysvar_id)
    }
}

impl AsAccount for AccountInfo<'_> {
    fn as_account<T>(&self, program_id: &Pubkey) -> Result<&T, ProgramError>
    where
        T: AccountDeserialize + Discriminator + Pod,
    {
        unsafe {
            self.has_owner(program_id)?;
            T::try_from_bytes(std::slice::from_raw_parts(
                self.try_borrow_data()?.as_ptr(),
                8 + std::mem::size_of::<T>(),
            ))
        }
    }

    fn as_account_mut<T>(&self, program_id: &Pubkey) -> Result<&mut T, ProgramError>
    where
        T: AccountDeserialize + Discriminator + Pod,
    {
        unsafe {
            self.has_owner(program_id)?;
            T::try_from_bytes_mut(std::slice::from_raw_parts_mut(
                self.try_borrow_mut_data()?.as_mut_ptr(),
                8 + std::mem::size_of::<T>(),
            ))
        }
    }
}

impl<'a, 'info> LamportTransfer<'a, 'info> for AccountInfo<'info> {
    #[inline(always)]
    fn send(&'a self, lamports: u64, to: &'a AccountInfo<'info>) {
        **self.lamports.borrow_mut() -= lamports;
        **to.lamports.borrow_mut() += lamports;
    }

    #[inline(always)]
    fn collect(&'a self, lamports: u64, from: &'a AccountInfo<'info>) -> Result<(), ProgramError> {
        solana_program::program::invoke(
            &solana_program::system_instruction::transfer(from.key, self.key, lamports),
            &[from.clone(), self.clone()],
        )
    }
}

impl<'a, 'info> CloseAccount<'a, 'info> for AccountInfo<'info> {
    fn close(&'a self, to: &'a AccountInfo<'info>) -> Result<(), ProgramError> {
        // Realloc data to zero.
        self.realloc(0, true)?;

        // Return rent lamports.
        self.send(self.lamports(), to);

        Ok(())
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

    fn assert_err<F>(
        &self,
        condition: F,
        err: solana_program::program_error::ProgramError,
    ) -> Result<&Self, solana_program::program_error::ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        if !condition(self) {
            return Err(err);
        }
        Ok(self)
    }

    fn assert_msg<F>(&self, condition: F, msg: &str) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        match crate::assert(
            condition(self),
            solana_program::program_error::ProgramError::InvalidAccountData,
            msg,
        ) {
            Err(err) => Err(err.into()),
            Ok(()) => Ok(self),
        }
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

    fn assert_err<F>(
        &self,
        condition: F,
        err: solana_program::program_error::ProgramError,
    ) -> Result<&Self, solana_program::program_error::ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        if !condition(self) {
            return Err(err);
        }
        Ok(self)
    }

    fn assert_msg<F>(&self, condition: F, msg: &str) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        match crate::assert(
            condition(self),
            solana_program::program_error::ProgramError::InvalidAccountData,
            msg,
        ) {
            Err(err) => Err(err.into()),
            Ok(()) => Ok(self),
        }
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
