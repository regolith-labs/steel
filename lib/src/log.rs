use pinocchio::error::ProgramError;
use pinocchio_log::log;

/// Logs a message.
#[inline(always)]
pub fn log(msg: &str) {
    log!("{}", msg);
}

/// Logs the call trace and returns the error.
#[track_caller]
pub fn trace(msg: &str, error: ProgramError) -> ProgramError {
    let caller = std::panic::Location::caller();
    log(&format!("{}: {}", msg, caller));
    error
}

/// Supports logging.
pub trait Loggable {
    fn log(&self);
    fn log_return(&self);
}
