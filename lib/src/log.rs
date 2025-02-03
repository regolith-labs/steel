use solana_program::{log::sol_log, program_error::ProgramError};

pub fn log(msg: String) {
    sol_log(msg.as_str());
}

/// Returns an error and logs the caller location.
#[track_caller]
pub fn trace(title: &str, msg: Option<&str>, error: ProgramError) -> ProgramError {
    let caller = std::panic::Location::caller();
    if let Some(msg) = msg {
        sol_log(format!("{}: {}: {}", title, msg, caller).as_str());
    } else {
        sol_log(format!("{}: {}", title, caller).as_str());
    }
    error
}
