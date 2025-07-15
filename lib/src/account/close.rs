use solana_program::{account_info::AccountInfo, program_error::ProgramError, system_program};

pub trait CloseAccount<'info> {
    fn close(&self, to: &AccountInfo<'info>) -> Result<(), ProgramError>;
}

impl<'info> CloseAccount<'info> for AccountInfo<'info> {
    fn close(&self, to: &AccountInfo<'info>) -> Result<(), ProgramError> {
        // Return rent lamports.
        **to.lamports.borrow_mut() += self.lamports();
        **self.lamports.borrow_mut() = 0;

        // Assign system program as the owner
        self.assign(&system_program::ID);

        // Realloc data to zero.
        self.realloc(0, true)?;

        Ok(())
    }
}
