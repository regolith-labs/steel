use pinocchio::{error::ProgramError, AccountView};
use pinocchio_system::instructions::Transfer;

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
        Transfer {
            from,
            to: &self,
            lamports,
        }
        .invoke()
    }
}
