mod initialize;

use example_0_api::instruction::MyInstruction;
use initialize::*;
use steel::*;

solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&example_0_api::id()) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (tag, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match MyInstruction::try_from(*tag).or(Err(ProgramError::InvalidInstructionData))? {
        MyInstruction::Initialize => process_initialize(accounts, data)?,
    }

    Ok(())
}
