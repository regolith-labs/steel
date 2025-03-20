use bytemuck::Pod;
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::trace;

use super::{AccountDeserialize, Discriminator};

pub trait AccountInfoValidation {
    fn is_signer(&self) -> Result<&Self, ProgramError>;
    fn is_writable(&self) -> Result<&Self, ProgramError>;
    fn is_executable(&self) -> Result<&Self, ProgramError>;
    fn is_empty(&self) -> Result<&Self, ProgramError>;
    fn is_type<T: Discriminator>(&self, program_id: &Pubkey) -> Result<&Self, ProgramError>;
    fn is_program(&self, program_id: &Pubkey) -> Result<&Self, ProgramError>;
    fn is_sysvar(&self, sysvar_id: &Pubkey) -> Result<&Self, ProgramError>;
    fn has_address(&self, address: &Pubkey) -> Result<&Self, ProgramError>;
    fn has_owner(&self, program_id: &Pubkey) -> Result<&Self, ProgramError>;
    fn has_seeds(&self, seeds: &[&[u8]], program_id: &Pubkey) -> Result<&Self, ProgramError>;
}

impl AccountInfoValidation for AccountInfo<'_> {
    #[track_caller]
    fn is_empty(&self) -> Result<&Self, ProgramError> {
        if !self.data_is_empty() {
            return Err(trace(
                "Account already initialized",
                ProgramError::AccountAlreadyInitialized,
            ));
        }
        Ok(self)
    }

    #[track_caller]
    fn is_executable(&self) -> Result<&Self, ProgramError> {
        if !self.executable {
            return Err(trace(
                "Account is not executable",
                ProgramError::InvalidAccountData,
            ));
        }
        Ok(self)
    }

    #[track_caller]
    fn is_program(&self, program_id: &Pubkey) -> Result<&Self, ProgramError> {
        self.has_address(program_id)?.is_executable()
    }

    #[track_caller]
    fn is_signer(&self) -> Result<&Self, ProgramError> {
        if !self.is_signer {
            return Err(trace(
                "Account is not a signer",
                ProgramError::MissingRequiredSignature,
            ));
        }
        Ok(self)
    }

    #[track_caller]
    fn is_sysvar(&self, sysvar_id: &Pubkey) -> Result<&Self, ProgramError> {
        self.has_owner(&solana_program::sysvar::ID)?
            .has_address(sysvar_id)
    }

    #[track_caller]
    fn is_type<T: Discriminator>(&self, program_id: &Pubkey) -> Result<&Self, ProgramError> {
        self.has_owner(program_id)?;
        if self.try_borrow_data()?[0].ne(&T::discriminator()) {
            return Err(trace(
                format!("Account is not of type {}", T::discriminator()).as_str(),
                ProgramError::InvalidAccountData,
            ));
        }
        Ok(self)
    }

    #[track_caller]
    fn is_writable(&self) -> Result<&Self, ProgramError> {
        if !self.is_writable {
            return Err(trace(
                "Account is not writable",
                ProgramError::MissingRequiredSignature,
            ));
        }
        Ok(self)
    }

    #[track_caller]
    fn has_address(&self, address: &Pubkey) -> Result<&Self, ProgramError> {
        if self.key.ne(address) {
            return Err(trace(
                "Account has invalid address",
                ProgramError::InvalidAccountData,
            ));
        }
        Ok(self)
    }

    #[track_caller]
    fn has_owner(&self, owner: &Pubkey) -> Result<&Self, ProgramError> {
        if self.owner.ne(owner) {
            return Err(trace(
                "Account has invalid owner",
                ProgramError::InvalidAccountOwner,
            ));
        }
        Ok(self)
    }

    #[track_caller]
    fn has_seeds(&self, seeds: &[&[u8]], program_id: &Pubkey) -> Result<&Self, ProgramError> {
        let pda = Pubkey::find_program_address(seeds, program_id);
        if self.key.ne(&pda.0) {
            return Err(trace(
                "Account has invalid seeds",
                ProgramError::InvalidSeeds,
            ));
        }
        Ok(self)
    }
}

/// Performs:
/// 1. Program owner check
/// 2. Discriminator byte check
/// 3. Checked bytemuck conversion of account data to &T or &mut T.
pub trait AsAccount {
    fn as_account<T>(&self, program_id: &Pubkey) -> Result<&T, ProgramError>
    where
        T: AccountDeserialize + Discriminator + Pod;

    fn as_account_mut<T>(&self, program_id: &Pubkey) -> Result<&mut T, ProgramError>
    where
        T: AccountDeserialize + Discriminator + Pod;
}

impl AsAccount for AccountInfo<'_> {
    #[track_caller]
    fn as_account<T>(&self, program_id: &Pubkey) -> Result<&T, ProgramError>
    where
        T: AccountDeserialize + Discriminator + Pod,
    {
        unsafe {
            // Validate account owner.
            self.has_owner(program_id)?;

            // Validate account data length.
            let data = self.try_borrow_data()?;
            let expected_len = 8 + std::mem::size_of::<T>();
            if data.len() != expected_len {
                return Err(ProgramError::InvalidAccountData);
            }

            // Deserialize account data.
            T::try_from_bytes(std::slice::from_raw_parts(data.as_ptr(), expected_len))
        }
    }

    #[track_caller]
    fn as_account_mut<T>(&self, program_id: &Pubkey) -> Result<&mut T, ProgramError>
    where
        T: AccountDeserialize + Discriminator + Pod,
    {
        unsafe {
            // Validate account owner.
            self.has_owner(program_id)?;

            // Validate account data length.
            let mut data = self.try_borrow_mut_data()?;
            let expected_len = 8 + std::mem::size_of::<T>();
            if data.len() != expected_len {
                return Err(ProgramError::InvalidAccountData);
            }

            // Deserialize account data.
            T::try_from_bytes_mut(std::slice::from_raw_parts_mut(
                data.as_mut_ptr(),
                expected_len,
            ))
        }
    }
}

pub trait AccountValidation {
    fn assert<F>(&self, condition: F) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool;

    fn assert_err<F>(&self, condition: F, err: ProgramError) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool;

    fn assert_msg<F>(&self, condition: F, msg: &str) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool;

    fn assert_mut<F>(&mut self, condition: F) -> Result<&mut Self, ProgramError>
    where
        F: Fn(&Self) -> bool;

    fn assert_mut_err<F>(
        &mut self,
        condition: F,
        err: ProgramError,
    ) -> Result<&mut Self, ProgramError>
    where
        F: Fn(&Self) -> bool;

    fn assert_mut_msg<F>(&mut self, condition: F, msg: &str) -> Result<&mut Self, ProgramError>
    where
        F: Fn(&Self) -> bool;
}
