use steel::*;

use crate::context::InitializeContext;

/// Initialize ...
pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let InitializeContext { signer: _ } = InitializeContext::load(accounts)?;

    // Return ok
    Ok(())
}
