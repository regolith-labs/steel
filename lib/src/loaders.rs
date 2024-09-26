#[cfg(feature = "spl")]
use solana_program::program_pack::Pack;
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};
#[cfg(feature = "spl")]
use spl_token::state::Mint;

use crate::Discriminator;

pub trait AccountInfoValidation {
    fn is_signer(&self) -> Result<&Self, ProgramError>;
    fn is_writable(&self) -> Result<&Self, ProgramError>;
    fn is_executable(&self) -> Result<&Self, ProgramError>;
    fn is_empty(&self) -> Result<&Self, ProgramError>;
    fn is_empty_pda(
        &self,
        seeds: &[&[u8]],
        bump: u8,
        program_id: &Pubkey,
    ) -> Result<&Self, ProgramError>;
    fn is_type<T: Discriminator>(&self) -> Result<&Self, ProgramError>;
    fn is_program(&self, program_id: &Pubkey) -> Result<&Self, ProgramError>;
    fn is_sysvar(&self, sysvar_id: &Pubkey) -> Result<&Self, ProgramError>;
    fn has_address(&self, address: &Pubkey) -> Result<&Self, ProgramError>;
    fn has_seeds(
        &self,
        seeds: &[&[u8]],
        bump: u8,
        program_id: &Pubkey,
    ) -> Result<&Self, ProgramError>;
    fn has_owner(&self, owner: &Pubkey) -> Result<&Self, ProgramError>;
}

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

    fn is_empty_pda(
        &self,
        seeds: &[&[u8]],
        bump: u8,
        program_id: &Pubkey,
    ) -> Result<&Self, ProgramError> {
        self.is_empty()?.has_seeds(seeds, bump, program_id)
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
        if self.key.ne(&address) {
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

/// Errors if:
/// - Owner is not SPL token program.
/// - Address does not match the expected mint address.
/// - Data is empty.
/// - Data cannot deserialize into a mint account.
/// - Expected to be writable, but is not.
#[cfg(feature = "spl")]
pub fn load_mint(
    info: &AccountInfo<'_>,
    address: Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&spl_token::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.key.ne(&address) {
        return Err(ProgramError::InvalidSeeds);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    Mint::unpack(&info.data.borrow())?;

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Owner is not SPL token program.
/// - Data is empty.
/// - Data cannot deserialize into a mint account.
/// - Expected to be writable, but is not.
#[cfg(feature = "spl")]
pub fn load_any_mint(info: &AccountInfo<'_>, is_writable: bool) -> Result<(), ProgramError> {
    if info.owner.ne(&spl_token::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    Mint::unpack(&info.data.borrow())?;

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Owner is not SPL token program.
/// - Data is empty.
/// - Data cannot deserialize into a token account.
/// - Token account owner does not match the expected owner address.
/// - Token account mint does not match the expected mint address.
/// - Expected to be writable, but is not.
#[cfg(feature = "spl")]
pub fn load_token_account(
    info: &AccountInfo<'_>,
    owner: Option<&Pubkey>,
    mint: &Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&spl_token::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    let account_data = info.data.borrow();
    let account = spl_token::state::Account::unpack(&account_data)?;

    if account.mint.ne(&mint) {
        return Err(ProgramError::InvalidAccountData);
    }

    if let Some(owner) = owner {
        if account.owner.ne(owner) {
            return Err(ProgramError::InvalidAccountData);
        }
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

/// Errors if:
/// - Owner is not SPL token program.
/// - Data is empty.
/// - Data cannot deserialize into a token account.
/// - Address does not match the expected associated token address.
/// - Expected to be writable, but is not.
#[cfg(feature = "spl")]
pub fn load_associated_token_account(
    info: &AccountInfo<'_>,
    owner: &Pubkey,
    mint: &Pubkey,
    is_writable: bool,
) -> Result<(), ProgramError> {
    if info.owner.ne(&spl_token::id()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if info.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    let account_data = info.data.borrow();
    let _ = spl_token::state::Account::unpack(&account_data)?;

    let address = spl_associated_token_account::get_associated_token_address(owner, mint);
    if info.key.ne(&address) {
        return Err(ProgramError::InvalidSeeds);
    }

    if is_writable && !info.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}
