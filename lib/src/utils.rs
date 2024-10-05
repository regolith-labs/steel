use bytemuck::Pod;
use solana_program::{
    entrypoint::ProgramResult, log::sol_log, program_error::ProgramError, pubkey::Pubkey,
};

/// Parses an instruction from the instruction data.
pub fn parse_instruction<'a, T: std::convert::TryFrom<u8>>(
    api_id: &'a Pubkey,
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

#[track_caller]
#[inline(always)]
pub fn assert_with_msg(v: bool, err: impl Into<ProgramError>, msg: &str) -> ProgramResult {
    if v {
        Ok(())
    } else {
        let caller = std::panic::Location::caller();
        sol_log(format!("{}. \n{}", msg, caller).as_str());
        Err(err.into())
    }
}

#[inline(always)]
pub fn try_cast_slice<T>(data: &[u8], expected_len: usize) -> Result<&[T], ProgramError>
where
    T: Pod,
{
    let expected_byte_len = std::mem::size_of::<T>() * expected_len;
    if data.len() != expected_byte_len {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }
    bytemuck::try_cast_slice(data).or(Err(
        solana_program::program_error::ProgramError::InvalidAccountData,
    ))
}

#[inline(always)]
pub fn try_cast_slice_mut<T>(data: &mut [u8], expected_len: usize) -> Result<&mut [T], ProgramError>
where
    T: Pod,
{
    let expected_byte_len = std::mem::size_of::<T>() * expected_len;
    if data.len() != expected_byte_len {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }
    bytemuck::try_cast_slice_mut(data).or(Err(
        solana_program::program_error::ProgramError::InvalidAccountData,
    ))
}

#[inline(always)]
pub fn try_cast_unsized_slice<T>(data: &[u8]) -> Result<&[T], ProgramError>
where
    T: Pod,
{
    bytemuck::try_cast_slice(data).or(Err(
        solana_program::program_error::ProgramError::InvalidAccountData,
    ))
}

#[inline(always)]
pub fn try_cast_unsized_slice_mut<T>(data: &mut [u8]) -> Result<&mut [T], ProgramError>
where
    T: Pod,
{
    bytemuck::try_cast_slice_mut(data).or(Err(
        solana_program::program_error::ProgramError::InvalidAccountData,
    ))
}

#[inline(always)]
pub fn try_cast_slice_with_remainder<T>(
    data: &[u8],
    expected_len: usize,
) -> Result<(&[T], &[u8]), ProgramError>
where
    T: Pod,
{
    let expected_byte_len = std::mem::size_of::<T>() * expected_len;
    let (head, remainder) = data.split_at(expected_byte_len);
    let head = bytemuck::try_cast_slice(head).or(Err(
        solana_program::program_error::ProgramError::InvalidAccountData,
    ))?;
    Ok((head, remainder))
}

#[inline(always)]
pub fn try_cast_slice_mut_with_remainder<T>(
    data: &mut [u8],
    expected_len: usize,
) -> Result<(&mut [T], &mut [u8]), ProgramError>
where
    T: Pod,
{
    let expected_byte_len = std::mem::size_of::<T>() * expected_len;
    let (head, remainder) = data.split_at_mut(expected_byte_len);
    let head = bytemuck::try_cast_slice_mut(head).or(Err(
        solana_program::program_error::ProgramError::InvalidAccountData,
    ))?;
    Ok((head, remainder))
}
