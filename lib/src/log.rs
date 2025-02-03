use solana_program::program_error::ProgramError;

/// Logs a message.
#[inline(always)]
pub fn log(msg: String) {
    solana_program::log::sol_log(msg.as_str());
}

/// Logs the call trace and returns the error.
#[track_caller]
pub fn trace(msg: &str, error: ProgramError) -> ProgramError {
    let caller = std::panic::Location::caller();
    log(format!("{}: {}", msg, caller));
    error
}
