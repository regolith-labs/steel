use crate::workspace::*;
use crate::BuildArgs;
use std::process::{Command, Stdio};

pub fn build_project(_args: BuildArgs) -> anyhow::Result<()> {
    verify_steel_workspace()?;

    Command::new("cargo")
        .arg("build-sbf")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute command");

    Ok(())
}
