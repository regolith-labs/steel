use solana_nostd_entrypoint::{
    solana_program::entrypoint::ProgramResult, AccountInfoC, InstructionC,
};

/// Use when the caller *has no* PDA signers.
#[inline]
pub fn cpi_invoke<const N: usize>(
    instruction: InstructionC,
    infos: &[AccountInfoC; N],
) -> ProgramResult {
    let pda_seeds: &[&[&[u8]]] = &[];

    #[cfg(target_os = "solana")]
    unsafe {
        solana_nostd_entrypoint::solana_program::syscalls::sol_invoke_signed_c(
            &instruction as *const InstructionC as *const u8,
            infos.as_ptr() as *const u8,
            infos.len() as u64,
            pda_seeds.as_ptr() as *const u8,
            0,
        );
    }

    // For clippy
    #[cfg(not(target_os = "solana"))]
    core::hint::black_box(&(&instruction, &infos, &pda_seeds));
    Ok(())
}

/// Use when the caller *has* PDA signers.
#[inline]
pub fn cpi_invoke_signed<const N: usize>(
    instruction: InstructionC,
    infos: &[AccountInfoC; N],
    pda_seeds: &[&[u8]],
) -> ProgramResult {
    // If balance is zero, create account
    #[cfg(target_os = "solana")]
    unsafe {
        solana_nostd_entrypoint::solana_program::syscalls::sol_invoke_signed_c(
            &instruction as *const InstructionC as *const u8,
            infos.as_ptr() as *const u8,
            infos.len() as u64,
            pda_seeds.as_ptr() as *const u8,
            pda_seeds.len() as u64,
        );
    }

    // For clippy
    #[cfg(not(target_os = "solana"))]
    core::hint::black_box(&(&instruction, &infos, &pda_seeds));
    Ok(())
}
