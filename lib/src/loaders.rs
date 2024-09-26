use bytemuck::Pod;
#[cfg(feature = "spl")]
use solana_program::program_pack::Pack;
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::{AccountDeserialize, AccountInfoValidation, Discriminator, ToAccount};
#[cfg(feature = "spl")]
use crate::{AccountValidation, ToSplToken};

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

    fn is_type<T: Discriminator>(&self) -> Result<&Self, ProgramError> {
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
        if self.key.ne(address) {
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
        if self.key.ne(&pda.0) || bump.ne(&pda.1) {
            return Err(ProgramError::InvalidSeeds);
        }
        Ok(self)
    }

    fn is_sysvar(&self, sysvar_id: &Pubkey) -> Result<&Self, ProgramError> {
        self.has_owner(&solana_program::sysvar::ID)?
            .has_address(sysvar_id)
    }
}

impl ToAccount for AccountInfo<'_> {
    fn to_account<T: AccountDeserialize + Discriminator + Pod>(&self) -> Result<&T, ProgramError> {
        unsafe {
            let data = self.try_borrow_data()?.as_ptr();
            T::try_from_bytes(std::slice::from_raw_parts(
                data,
                8 + std::mem::size_of::<T>(),
            ))
        }
    }

    fn to_account_mut<T: AccountDeserialize + Discriminator + Pod>(
        &self,
    ) -> Result<&mut T, ProgramError> {
        self.is_writable()?;
        unsafe {
            let data = self.try_borrow_mut_data()?.as_mut_ptr();
            T::try_from_bytes_mut(std::slice::from_raw_parts_mut(
                data,
                8 + std::mem::size_of::<T>(),
            ))
        }
    }
}

#[cfg(feature = "spl")]
impl ToSplToken for AccountInfo<'_> {
    fn to_mint(&self) -> Result<spl_token::state::Mint, ProgramError> {
        unsafe {
            self.has_owner(&spl_token::ID)?;
            let data = self.try_borrow_data()?.as_ptr();
            spl_token::state::Mint::unpack(std::slice::from_raw_parts(
                data,
                spl_token::state::Mint::LEN,
            ))
        }
    }
    fn to_token_account(&self) -> Result<spl_token::state::Account, ProgramError> {
        unsafe {
            self.has_owner(&spl_token::ID)?;
            let data = self.try_borrow_data()?.as_ptr();
            spl_token::state::Account::unpack(std::slice::from_raw_parts(
                data,
                spl_token::state::Account::LEN,
            ))
        }
    }
}

#[cfg(feature = "spl")]
impl AccountValidation for spl_token::state::Mint {
    fn check<F>(&self, condition: F) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        if !condition(self) {
            return Err(solana_program::program_error::ProgramError::InvalidAccountData);
        }
        Ok(self)
    }

    fn check_mut<F>(&mut self, _condition: F) -> Result<&mut Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        panic!("not implemented")
    }
}

#[cfg(feature = "spl")]
impl AccountValidation for spl_token::state::Account {
    fn check<F>(&self, condition: F) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        if !condition(self) {
            return Err(solana_program::program_error::ProgramError::InvalidAccountData);
        }
        Ok(self)
    }

    fn check_mut<F>(&mut self, _condition: F) -> Result<&mut Self, ProgramError>
    where
        F: Fn(&Self) -> bool,
    {
        panic!("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use bytemuck::{Pod, Zeroable};
    use num_enum::{IntoPrimitive, TryFromPrimitive};
    use solana_program::{
        account_info::AccountInfo, loader_v4, program_error::ProgramError, pubkey::Pubkey,
        rent::Rent,
    };

    use crate::{account, AccountInfoValidation, Discriminator};

    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
    enum Accounts {
        TestAccount = 0,
        OtherAccount = 1,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
    struct TestAccount {
        discriminator: u8,
        data: [u8; 32],
    }
    account!(Accounts, TestAccount);

    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
    struct OtherAccount {
        discriminator: u8,
        padding: [u8; 7],
        data: u64,
    }
    account!(Accounts, OtherAccount);

    #[test]
    fn is_signer() {
        let key = Pubkey::new_unique();
        let owner_key = Pubkey::default();
        let lamports = &mut 10_000_000_000;
        let data: &mut [u8] = &mut [0u8; 0];

        // Case: is not signer
        let account_info = AccountInfo::new(
            &owner_key, false, false, lamports, data, &owner_key, false, 0,
        );
        assert!(
            account_info.is_signer().err().unwrap() == ProgramError::MissingRequiredSignature,
            "Expected account to not be a signer"
        );

        // Case: is signer
        let account_info = AccountInfo::new(&key, true, false, lamports, data, &key, false, 0);
        assert!(
            account_info.is_signer().is_ok(),
            "Expected account to not be a signer"
        );
    }

    #[test]
    fn is_writable() {
        let key = Pubkey::new_unique();
        let owner_key = Pubkey::default();
        let lamports = &mut 0;
        let data: &mut [u8] = &mut [255u8; 64];

        // Case: is not writable
        let account_info =
            AccountInfo::new(&key, false, false, lamports, data, &owner_key, false, 0);
        assert!(
            account_info.is_writable().err().unwrap() == ProgramError::MissingRequiredSignature,
            "Expected account to not be writable"
        );

        // Case: is writable
        let account_info = AccountInfo::new(&key, true, true, lamports, data, &owner_key, false, 0);
        assert!(
            account_info.is_writable().is_ok(),
            "Expected account to be writable"
        );
    }

    #[test]
    fn is_executable() {
        let key = Pubkey::new_unique();
        let owner_key = Pubkey::default();
        let lamports = &mut 10_000_000_000;
        let data: &mut [u8] = &mut [0u8; 64];

        // Case: is not executable
        let account_info =
            AccountInfo::new(&key, false, false, lamports, data, &owner_key, false, 0);
        assert!(
            account_info.is_executable().err().unwrap() == ProgramError::InvalidAccountData,
            "Expected account to not be executable"
        );

        // Case: is executable
        let account_info = AccountInfo::new(&key, false, true, lamports, data, &owner_key, true, 0);
        assert!(
            account_info.is_executable().is_ok(),
            "Expected account to be executable"
        );
    }

    #[test]
    fn is_empty() {
        let key = Pubkey::new_unique();
        let owner_key = Pubkey::default();
        let lamports = &mut 0;
        let empty_data: &mut [u8] = &mut [0u8; 0];

        // Case: is not empty
        let data: &mut [u8] = &mut [0u8; 33];
        let test_account = TestAccount {
            discriminator: Accounts::TestAccount as u8,
            data: [255u8; 32],
        };
        data.copy_from_slice(test_account.to_bytes().as_ref());

        let account_info =
            AccountInfo::new(&key, false, false, lamports, data, &owner_key, false, 0);
        assert!(
            account_info.is_empty().err().unwrap() == ProgramError::AccountAlreadyInitialized,
            "Expected account to not be empty"
        );

        // Case: is empty
        let account_info = AccountInfo::new(
            &key, false, false, lamports, empty_data, &owner_key, false, 0,
        );
        assert!(
            account_info.is_empty().is_ok(),
            "Expected account to be empty"
        );
    }

    #[test]
    fn is_program() {
        let wrong_key = Pubkey::new_unique();
        let program_key = Pubkey::new_unique();
        let owner_key = loader_v4::ID;
        let lamports = &mut 0;
        let data: &mut [u8] = &mut [0u8; 1];

        // Case: is not address but is executable
        let account_info = AccountInfo::new(
            &wrong_key, false, false, lamports, data, &owner_key, true, 0,
        );
        assert!(
            account_info.is_program(&program_key).err().unwrap()
                == ProgramError::InvalidAccountData,
            "Expected account to pass validation due to incorrect address"
        );

        // Case: is address but is not executable
        let account_info = AccountInfo::new(
            &program_key,
            true,
            true,
            lamports,
            data,
            &owner_key,
            false,
            0,
        );
        assert!(
            account_info.is_program(&program_key).err().unwrap()
                == ProgramError::InvalidAccountData,
            "Expected account to not be executable"
        );

        // Case: is address and is executable
        let account_info = AccountInfo::new(
            &program_key,
            true,
            true,
            lamports,
            data,
            &owner_key,
            true,
            0,
        );
        assert!(
            account_info.is_program(&program_key).is_ok(),
            "Expected account to be a program"
        );
    }

    #[test]
    fn is_type() {
        let key = Pubkey::new_unique();
        let owner_key = Pubkey::default();

        // Case: is not correct type, is OtherAccount
        let lamports = &mut Rent::default().minimum_balance(std::mem::size_of::<OtherAccount>());
        let data: &mut [u8] = &mut [0u8; std::mem::size_of::<OtherAccount>()];
        let other_account = OtherAccount {
            discriminator: Accounts::OtherAccount as u8,
            padding: [255u8; 7],
            data: u64::MAX,
        };
        data.copy_from_slice(other_account.to_bytes().as_ref());

        let account_info =
            AccountInfo::new(&key, false, false, lamports, data, &owner_key, false, 0);
        assert!(
            account_info.is_type::<TestAccount>().err().unwrap()
                == ProgramError::InvalidAccountData,
            "Expected account to not be of type TestAccount but was"
        );

        // Case: is correct type, is TestAccount
        let lamports = &mut Rent::default().minimum_balance(std::mem::size_of::<TestAccount>());
        let data: &mut [u8] = &mut [0u8; std::mem::size_of::<TestAccount>()];
        let test_account = TestAccount {
            discriminator: Accounts::TestAccount as u8,
            data: [255u8; 32],
        };
        data.copy_from_slice(test_account.to_bytes().as_ref());

        let account_info =
            AccountInfo::new(&key, false, false, lamports, data, &owner_key, false, 0);
        assert!(
            account_info.is_type::<TestAccount>().is_ok(),
            "Expected account to be of type TestAccount"
        );
    }

    #[test]
    fn has_owner() {
        let key = Pubkey::new_unique();
        let owner_key = Pubkey::default();
        let lamports = &mut 0;
        let data: &mut [u8] = &mut [0u8; 1];

        // Case: is not owner
        let wrong_key = Pubkey::new_unique();
        let account_info =
            AccountInfo::new(&key, false, false, lamports, data, &wrong_key, false, 0);
        assert!(
            account_info.has_owner(&owner_key).err().unwrap() == ProgramError::InvalidAccountOwner,
            "Expected account to not be owned by the provided owner key"
        );

        // Case: is owner
        let account_info = AccountInfo::new(&key, false, false, lamports, data, &key, false, 0);
        assert!(
            account_info.has_owner(&key).is_ok(),
            "Expected account to be owned by the owner key"
        );
    }

    #[test]
    fn has_address() {
        let key = Pubkey::new_unique();
        let owner_key = Pubkey::default();
        let lamports = &mut 0;
        let data: &mut [u8] = &mut [0u8; 1];

        // Case: is not address
        let wrong_key = Pubkey::new_unique();
        let account_info = AccountInfo::new(
            &wrong_key, false, false, lamports, data, &owner_key, false, 0,
        );
        assert!(
            account_info.has_address(&key).is_err(),
            "Expected account to not have the address"
        );

        // Case: is address
        let account_info =
            AccountInfo::new(&key, true, false, lamports, data, &owner_key, false, 0);
        assert!(
            account_info.has_address(&key).is_ok(),
            "Expected account to have the address"
        );
    }

    #[test]
    fn has_seeds() {
        let program_key = loader_v4::ID;
        let (key, bump) = Pubkey::find_program_address(&[b"seed", &[69]], &program_key);
        let owner_key = Pubkey::default();
        let lamports = &mut 0;
        let data: &mut [u8] = &mut [0u8; 1];

        // Case: Correct seeds, incorrect bump.
        let account_info =
            AccountInfo::new(&key, false, false, lamports, data, &owner_key, false, 0);
        assert!(
            account_info
                .has_seeds(&[b"seed", &[69]], bump - 1, &program_key)
                .err()
                .unwrap()
                == ProgramError::InvalidSeeds,
            "Expected account not to derive from program derived address with incorrect bump"
        );

        // Case: Incorrect seeds, correct bump.
        let account_info =
            AccountInfo::new(&key, false, false, lamports, data, &owner_key, false, 0);
        assert!(
            account_info
                .has_seeds(&[b"seed", &[255]], bump, &program_key)
                .err()
                .unwrap()
                == ProgramError::InvalidSeeds,
            "Expected account not to derive from program derived address with incorrect seeds"
        );

        // Case: Correct seeds and bump, incorrect program.
        let wrong_key = Pubkey::new_unique();
        let account_info =
            AccountInfo::new(&key, false, false, lamports, data, &owner_key, false, 0);
        assert!(
            account_info
                .has_seeds(&[b"seed", &[255]], bump, &wrong_key)
                .err()
                .unwrap()
                == ProgramError::InvalidSeeds,
            "Expected account not to derive from program derived address with incorrect program"
        );

        // Case: Correct seeds, Correct bump.
        let account_info =
            AccountInfo::new(&key, false, false, lamports, data, &owner_key, false, 0);
        assert!(
            account_info
                .has_seeds(&[b"seed", &[69]], bump, &program_key)
                .is_ok(),
            "Expected account to derive from program derived address when given correct seed, bump"
        );
    }

    #[test]
    fn is_sysvar() {
        let key = Pubkey::new_unique();
        let owner_key = solana_program::sysvar::ID;
        let lamports = &mut 0;
        let data: &mut [u8] = &mut [0u8; 1];

        // Case: is not sysvar owned, is not correct key
        let wrong_owner_key = Pubkey::new_unique();
        let wrong_account_key = Pubkey::new_unique();
        let account_info = AccountInfo::new(
            &wrong_account_key,
            false,
            false,
            lamports,
            data,
            &wrong_owner_key,
            false,
            0,
        );

        assert!(
            account_info.is_sysvar(&key).err().unwrap() == ProgramError::InvalidAccountOwner,
            "Expected account to not be a sysvar"
        );

        // Case: is not sysvar owned, is correct key.
        let wrong_key = Pubkey::new_unique();
        let account_info =
            AccountInfo::new(&key, false, false, lamports, data, &wrong_key, false, 0);
        assert!(
            account_info.is_sysvar(&key).err().unwrap() == ProgramError::InvalidAccountOwner,
            "Expected account to not be a sysvar"
        );

        // Case: is owned by sysvar, is correct key.
        let account_info =
            AccountInfo::new(&key, false, false, lamports, data, &owner_key, false, 0);
        assert!(
            account_info.is_sysvar(&key).is_ok(),
            "Expected account to be a sysvar"
        );
    }

    #[test]
    fn compound_validation() -> Result<(), ProgramError> {
        let key = Pubkey::new_unique();
        let owner_key = Pubkey::default();
        let lamports = &mut Rent::default().minimum_balance(std::mem::size_of::<TestAccount>());
        let data: &mut [u8] = &mut [0u8; std::mem::size_of::<TestAccount>()];
        let test_account = TestAccount {
            discriminator: Accounts::TestAccount as u8,
            data: [255u8; 32],
        };

        data.copy_from_slice(test_account.to_bytes().as_ref());

        let account_info =
            AccountInfo::new(&key, false, true, lamports, data, &owner_key, false, 0);

        assert!(
            account_info
                .is_type::<TestAccount>()?
                .is_writable()?
                .is_executable()
                .err()
                .unwrap()
                == ProgramError::InvalidAccountData
        );

        assert!(account_info.is_writable()?.is_type::<TestAccount>().is_ok());

        Ok(())
    }
}
