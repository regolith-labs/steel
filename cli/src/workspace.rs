// src/utils.rs or src/workspace.rs
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn verify_steel_workspace() -> Result<PathBuf> {
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;

    let cargo_toml_path = current_dir.join("Cargo.toml");

    if !cargo_toml_path.exists() {
        anyhow::bail!("Error: Not in a Steel workspace.\n\nNo Cargo.toml found. Run 'steel new <name>' to create a new Steel project.");
    }

    let cargo_toml_content =
        fs::read_to_string(&cargo_toml_path).context("Failed to read Cargo.toml")?;

    if !cargo_toml_content.contains("steel") {
        anyhow::bail!("Error: Not in a Steel workspace.\n\n. Run 'steel new <name>' to create a new Steel project.");
    }

    Ok(current_dir)
}
