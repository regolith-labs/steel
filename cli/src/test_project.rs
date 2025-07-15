use std::process::{Command, Stdio};

use crate::TestArgs;

pub fn test_project(args: TestArgs) -> anyhow::Result<()> {
    let mut command = Command::new("cargo");
        command.arg("test-sbf");

        if args.nocapture {
            command.arg("--").arg("--nocapture");
        }

        command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute command");

    Ok(())
}
