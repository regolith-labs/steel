use bytemuck::Pod;
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

pub trait AccountDeserialize {
    fn try_from_bytes(data: &[u8]) -> Result<&Self, ProgramError>;
    fn try_from_bytes_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError>;
}

pub trait AccountValidation {
    fn check<F>(&self, condition: F) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool;

    fn check_mut<F>(&mut self, condition: F) -> Result<&mut Self, ProgramError>
    where
        F: Fn(&Self) -> bool;
}

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

pub trait Discriminator {
    fn discriminator() -> u8;
}

pub trait ToAccount {
    fn to_account<T: AccountDeserialize + Discriminator + Pod>(&self) -> Result<&T, ProgramError>;
    fn to_account_mut<T: AccountDeserialize + Discriminator + Pod>(
        &self,
    ) -> Result<&mut T, ProgramError>;
}
