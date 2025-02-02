use solana_program::log::sol_log;

pub fn log(msg: String) {
    sol_log(msg.as_str());
}
