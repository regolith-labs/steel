use solana_program::{account_info::AccountInfo, program_error::ProgramError};

use crate::LamportTransfer;

pub trait CloseAccount<'a, 'info> {
    fn close(&'a self, to: &'a AccountInfo<'info>) -> Result<(), ProgramError>;
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
