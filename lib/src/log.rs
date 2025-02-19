#[cfg(not(feature = "pinocchio"))]
use solana_program::program_error::ProgramError;

#[cfg(feature = "pinocchio")]
use pinocchio::program_error::ProgramError;

/// Logs a message.
#[cfg(not(feature = "pinocchio"))]
#[inline(always)]
pub fn log(msg: String) {
    solana_program::log::sol_log(msg.as_str());
}

/// Logs the call trace and returns the error.
#[cfg(not(feature = "pinocchio"))]
#[track_caller]
pub fn trace(msg: &str, error: ProgramError) -> ProgramError {
    let caller = std::panic::Location::caller();
    log(format!("{}: {}", msg, caller));
    error
}

/// Logs a message.
#[cfg(feature = "pinocchio")]
#[inline(always)]
pub fn log(msg: String) {
    pinocchio_log::log!("{}", msg.as_str());
}

/// Logs the call trace and returns the error.
#[cfg(feature = "pinocchio")]
#[track_caller]
pub fn trace(msg: &str, error: ProgramError) -> ProgramError {
    let caller = std::panic::Location::caller();
    log(format!("{}: {}", msg, caller));
    error
}
