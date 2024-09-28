use steel::*;

/// Initialize ...
pub fn process_initialize(
    accounts: &ArrayVec<NoStdAccountInfo, 32>,
    _data: &[u8],
) -> ProgramResult {
    // Load accounts.
    let [signer, ..] = accounts.as_slice() else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    AccountInfoValidation::is_signer(signer)?;

    // Return ok
    Ok(())
}
