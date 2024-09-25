mod context;
mod initialize;

use example_0_api::instruction::MyInstruction;
use initialize::*;
use steel::*;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction::<MyInstruction>(example_0_api::id(), program_id, data)?;

    match ix {
        MyInstruction::Initialize => process_initialize(accounts, data)?,
    }

    Ok(())
}
