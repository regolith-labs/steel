use std::process::{Command, Stdio};

use crate::CleanArgs;

pub fn clean_project(_args: CleanArgs) -> anyhow::Result<()> {
    Command::new("cargo")
        .arg("clean")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute command");

    Ok(())
}
