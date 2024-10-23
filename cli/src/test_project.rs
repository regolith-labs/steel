use std::process::{Command, Stdio};

use crate::TestArgs;

pub fn test_project(_args: TestArgs) -> anyhow::Result<()> {
    Command::new("cargo")
        .arg("test-sbf")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute command");

    Ok(())
}
