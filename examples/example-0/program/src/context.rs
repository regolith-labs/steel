use solana_program::account_info::next_account_info;
use steel::*;

pub(crate) struct InitializeContext<'a, 'info> {
    pub signer: Signer<'a, 'info>,
}

impl<'a, 'info> InitializeContext<'a, 'info> {
    pub(crate) fn load(accounts: &'a [AccountInfo<'info>]) -> Result<Self, ProgramError> {
        let accounts = &mut accounts.iter();

        let signer = Signer::new(next_account_info(accounts)?)?;

        Ok(Self { signer })
    }
}
