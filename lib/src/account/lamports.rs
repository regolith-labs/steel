use pinocchio_system::instructions::Transfer;
// use solana_program::{account_info::AccountInfo, program_error::ProgramError};
use solana_account_view::AccountView;
use solana_program_error::ProgramError;

pub trait LamportTransfer {
    fn send(&self, lamports: u64, to: &AccountView);
    fn collect(&self, lamports: u64, from: &AccountView) -> Result<(), ProgramError>;
}

impl LamportTransfer for AccountView {
    #[inline(always)]
    fn send(&self, lamports: u64, to: &AccountView) {
        self.set_lamports(self.lamports() - lamports);
        to.set_lamports(to.lamports() + lamports);
    }

    #[inline(always)]
    fn collect(&self, lamports: u64, from: &AccountView) -> Result<(), ProgramError> {
        // Transfer { from, to, lamports }.invoke()?;
        // solana_instruction_view::cpi::invoke::<2>(instruction, &[from, self])
        // solana_program::program::invoke(
        //     &solana_program::system_instruction::transfer(from.key, self.key, lamports),
        //     &[from.clone(), self.clone()],
        // )
        Ok(())
    }
}
