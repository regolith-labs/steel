use solana_account_view::AccountView;
use solana_address::address;
use solana_program_error::ProgramError;

pub trait CloseAccount {
    fn close(&self, to: &AccountView) -> Result<(), ProgramError>;
}

impl CloseAccount for AccountView {
    fn close(&self, to: &AccountView) -> Result<(), ProgramError> {
        // Return rent lamports.
        to.set_lamports(to.lamports() + self.lamports());
        self.set_lamports(0);

        // Assign system program as the owner
        unsafe {
            // self.assign(&system_program::ID);
            self.assign(&address!("11111111111111111111111111111111"));
        }

        // Realloc data to zero.
        self.resize(0)?;

        Ok(())
    }
}
