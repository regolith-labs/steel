use std::process::{Command, Stdio};

use crate::BuildArgs;

pub fn build_project(_args: BuildArgs) -> anyhow::Result<()> {
    Command::new("cargo")
        .arg("build-sbf")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute command");

    Ok(())
}
