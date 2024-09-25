use solana_program::{program_error::ProgramError, pubkey::Pubkey};

/// Parses an instruction from the instruction data.
pub fn parse_instruction<'a, T: std::convert::TryFrom<u8>>(
    api_id: Pubkey,
    program_id: &'a Pubkey,
    data: &'a [u8],
) -> Result<(T, &'a [u8]), ProgramError> {
    // Validate the program id is valid.
    if program_id.ne(&api_id) {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Parse data for instruction discriminator.
    let (tag, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    // Get instruction for discriminator.
    let ix = T::try_from(*tag).or(Err(ProgramError::InvalidInstructionData))?;

    // Return
    Ok((ix, data))
}
