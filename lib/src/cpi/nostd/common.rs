use solana_nostd_entrypoint::NoStdAccountInfo;
use solana_program::{
    entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey, rent::Rent,
    sysvar::Sysvar,
};

use super::system_program::{allocate, assign, create_account, transfer};

/// Creates a new pda.
#[inline(always)]
pub fn create_pda<'a, 'info>(
    target_account: NoStdAccountInfo,
    owner: &Pubkey,
    space: usize,
    pda_seeds: &[&[u8]],
    system_program: NoStdAccountInfo,
    payer: NoStdAccountInfo,
) -> ProgramResult {
    let rent = Rent::get()?;
    let lamports = target_account
        .try_borrow_lamports()
        .ok_or(ProgramError::AccountBorrowFailed)?;
    if lamports.eq(&0) {
        // If balance is zero, create account
        create_account(
            &payer,
            &target_account,
            &system_program,
            &lamports,
            &(space as u64),
            owner,
        )?;
    } else {
        // Otherwise, if balance is nonzero:

        // 1) transfer sufficient lamports for rent exemption
        let rent_exempt_balance = rent.minimum_balance(space).saturating_sub(*lamports);
        if rent_exempt_balance.gt(&0) {
            transfer(
                &payer,
                &target_account,
                &system_program,
                &rent_exempt_balance,
            )?;
        }

        // 2) allocate space for the account
        allocate(&target_account, &system_program, &(space as u64), pda_seeds)?;

        // 3) assign our program as the owner
        assign(&target_account, owner, &system_program, pda_seeds)?;
    }

    Ok(())
}
