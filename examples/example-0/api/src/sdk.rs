use steel::*;

use crate::instruction::*;

/// Builds an initialize instruction.
pub fn initialize(signer: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::id(),
        accounts: vec![AccountMeta::new(signer, true)],
        data: Initialize {}.to_bytes(),
    }
}
