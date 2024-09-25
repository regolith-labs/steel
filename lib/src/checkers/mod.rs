use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
    sysvar::{self},
};

mod token;

use crate::assert_with_msg;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Signer<'a, 'info> {
    info: &'a AccountInfo<'info>,
}

impl<'a, 'info> Signer<'a, 'info> {
    pub fn new(info: &'a AccountInfo<'info>) -> Result<Signer<'a, 'info>, ProgramError> {
        assert_with_msg(
            info.is_signer,
            ProgramError::MissingRequiredSignature,
            "Missing required signature",
        )?;
        Ok(Self { info })
    }

    pub fn new_with_key(
        info: &'a AccountInfo<'info>,
        key: &Pubkey,
    ) -> Result<Signer<'a, 'info>, ProgramError> {
        let signer = Self::new(info)?;
        assert_with_msg(
            signer.info.key == key,
            ProgramError::MissingRequiredSignature,
            "Incorrect key for signer",
        )?;
        Ok(signer)
    }

    pub fn new_payer(info: &'a AccountInfo<'info>) -> Result<Signer<'a, 'info>, ProgramError> {
        assert_with_msg(
            info.is_writable,
            ProgramError::InvalidInstructionData,
            "Payer is not writable",
        )?;
        assert_with_msg(
            info.is_signer,
            ProgramError::MissingRequiredSignature,
            "Missing required signature for payer",
        )?;
        Ok(Self { info })
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct PDA<'a, 'info> {
    info: &'a AccountInfo<'info>,
}

impl<'a, 'info> PDA<'a, 'info> {
    pub fn new_with_known_address(
        info: &'a AccountInfo<'info>,
        known_address: &Pubkey,
    ) -> Result<PDA<'a, 'info>, ProgramError> {
        assert_with_msg(
            info.key == known_address,
            ProgramError::InvalidInstructionData,
            "Incorrect account key",
        )?;
        Ok(Self { info })
    }

    pub fn new_with_seeds(
        info: &'a AccountInfo<'info>,
        seeds: &[&[u8]],
        bump: u8,
        program_id: &Pubkey,
    ) -> Result<PDA<'a, 'info>, ProgramError> {
        let pda = Pubkey::find_program_address(seeds, program_id);

        assert_with_msg(
            info.key == &pda.0,
            ProgramError::InvalidSeeds,
            "invalid pda seeds",
        )?;
        assert_with_msg(
            bump == pda.1,
            ProgramError::InvalidSeeds,
            "invalid pda bump",
        )?;

        Ok(Self { info })
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct EmptyAccount<'a, 'info> {
    info: &'a AccountInfo<'info>,
}

impl<'a, 'info> EmptyAccount<'a, 'info> {
    pub fn new(info: &'a AccountInfo<'info>) -> Result<EmptyAccount<'a, 'info>, ProgramError> {
        assert_with_msg(
            info.data_is_empty(),
            ProgramError::InvalidAccountData,
            "Account must be uninitialized",
        )?;
        assert_with_msg(
            info.owner == &system_program::id(),
            ProgramError::IllegalOwner,
            "Empty accounts must be owned by the system program",
        )?;
        Ok(Self { info })
    }

    pub fn new_with_key(
        info: &'a AccountInfo<'info>,
        key: Pubkey,
        is_writable: bool,
    ) -> Result<EmptyAccount<'a, 'info>, ProgramError> {
        assert_with_msg(
            info.data_is_empty(),
            ProgramError::InvalidAccountData,
            "Account must be uninitialized",
        )?;
        assert_with_msg(
            info.owner == &system_program::id(),
            ProgramError::IllegalOwner,
            "Empty accounts must be owned by the system program",
        )?;
        assert_with_msg(
            info.key == &key,
            ProgramError::InvalidAccountData,
            "Invalid validation pubkey",
        )?;
        assert_with_msg(
            is_writable == info.is_writable,
            ProgramError::InvalidAccountData,
            "account should/shouldn't be writiable",
        )?;
        Ok(Self { info })
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct SysvarAccount<'a, 'info> {
    info: &'a AccountInfo<'info>,
}

impl<'a, 'info> SysvarAccount<'a, 'info> {
    pub fn new(info: &'a AccountInfo<'info>) -> Result<SysvarAccount<'a, 'info>, ProgramError> {
        assert_with_msg(
            info.owner == &sysvar::id(),
            ProgramError::InvalidAccountOwner,
            "Account must be owned by sysvar",
        )?;

        Ok(Self { info })
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct ProgramAccount<'a, 'info> {
    info: &'a AccountInfo<'info>,
}

impl<'a, 'info> ProgramAccount<'a, 'info> {
    pub fn new(
        info: &'a AccountInfo<'info>,
        program_id: &Pubkey,
    ) -> Result<ProgramAccount<'a, 'info>, ProgramError> {
        assert_with_msg(
            info.owner == program_id,
            ProgramError::InvalidAccountOwner,
            "Account must be owned by program pubkey passed in",
        )?;

        assert_with_msg(
            info.executable == false,
            ProgramError::InvalidAccountOwner,
            "Account should be executable",
        )?;

        Ok(Self { info })
    }
}