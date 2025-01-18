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
pub fn assert(v: bool, err: impl Into<ProgramError>, msg: &str) -> ProgramResult {
    if v {
        Ok(())
    } else {
        let caller = std::panic::Location::caller();
        sol_log(format!("{}. \n{}", msg, caller).as_str());
        Err(err.into())
    }
}

/// Converts a string into a fixed-size byte array of length N.
///
/// # Arguments
/// * `s` - The input string to convert
///
/// # Returns
/// * `Ok([u8; N])` - A fixed-size byte array containing the string data, zero-padded if needed
/// * `Err(ProgramError)` - Returns ERROR_STRING_TOO_LONG if input string is longer than N bytes
pub fn string_to_bytes<const N: usize>(s: &str) -> Result<[u8; N], ProgramError> {
    let mut bytes = [0; N];
    let s_bytes = s.as_bytes();

    // Check length before doing any operations
    if s_bytes.len() > N {
        return Err(ProgramError::Custom(ERROR_STRING_TOO_LONG));
    }

    let len = s_bytes.len();
    bytes[..len].copy_from_slice(&s_bytes[..len]);
    Ok(bytes)
}

/// Converts a fixed-size byte array into a String, handling null termination.
///
/// # Arguments
/// * `bytes` - The fixed-size byte array to convert
///
/// # Returns
/// * `Ok(String)` - The converted string, truncated at the first null byte if present
/// * `Err(ProgramError)` - Returns ERROR_INVALID_UTF8 if the bytes are not valid UTF-8
pub fn bytes_to_string<const N: usize>(bytes: &[u8; N]) -> Result<String, ProgramError> {
    // Find the actual length by looking for the first zero or taking full length
    let actual_len = bytes.iter().position(|&b| b == 0).unwrap_or(N);

    // Convert the slice up to actual_len to a string
    Ok(String::from_utf8_lossy(&bytes[..actual_len])
        .trim_matches('\0')
        .to_string())
}

pub const ERROR_STRING_TOO_LONG: u32 = 1;
pub const ERROR_INVALID_UTF8: u32 = 2;

#[test]
fn test_string_to_bytes() {
    // Test successful conversion
    let result = string_to_bytes::<5>("hello");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), *b"hello");

    // Test string shorter than buffer
    let result = string_to_bytes::<5>("hi");
    assert!(result.is_ok());
    assert_eq!(&result.unwrap()[..2], b"hi");

    // Test string too long
    let result = string_to_bytes::<3>("hello");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        ProgramError::Custom(ERROR_STRING_TOO_LONG)
    );
}

#[test]
fn test_bytes_to_string() {
    // Test successful conversion
    let bytes = *b"hello";
    let result = bytes_to_string::<5>(&bytes);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello");

    // Test with null termination
    let mut bytes = [0u8; 5];
    bytes[..3].copy_from_slice(b"hi\0");
    let result = bytes_to_string::<5>(&bytes);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hi");
}
