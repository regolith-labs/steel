mod initialize;

use example_1_api::instruction::MyInstruction;
use initialize::*;
use steel::*;

entrypoint_nostd!(process_instruction, 32);

noalloc_allocator!();
basic_panic_impl!();

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &ArrayVec<NoStdAccountInfo, 32>,
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction::<MyInstruction>(&example_1_api::ID, program_id, data)?;

    match ix {
        MyInstruction::Initialize => process_initialize(accounts, data)?,
    }

    Ok(())
}
