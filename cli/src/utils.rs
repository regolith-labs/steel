use solana_sdk::{signature::Keypair, signer::Signer};
use std::{
    fs::{self},
    io::Write,
};
use toml::Value;

pub fn prompt(prompt: &str) -> String {
    println!("{}", prompt);

    // Read input
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim the newline character from the end of the input
    let input = input.trim();
    input.to_owned()
}

pub fn to_camel_case(input: &str) -> String {
    let normalized_input = input.replace("-", " ").replace("_", " ");
    let mut words = normalized_input.split_whitespace();
    let mut camel_case_string = String::new();

    if let Some(first_word) = words.next() {
        camel_case_string.push_str(&first_word.to_lowercase());
    }

    for word in words {
        let mut chars = word.chars();
        if let Some(first_char) = chars.next() {
            camel_case_string.push(first_char.to_ascii_uppercase());
            camel_case_string.push_str(&chars.as_str().to_lowercase());
        }
    }

    camel_case_string
}

pub fn to_type_case(input: &str) -> String {
    capitalize_first(&to_camel_case(input))
}

pub fn to_lib_case(input: &str) -> String {
    input.to_ascii_lowercase().replace("-", "_")
}

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Determines if valid steel project.
///
/// checks:
/// 1. The folder structure contains `api` & `program` dirs.
/// 2. The presence of the `steel` dependency in `Cargo.toml`.
///
/// # Returns
/// - `Ok(true)` if the project is a valid Steel project.
/// - `Ok(false)` if the project is not a valid Steel project.
/// - `Err` if `Cargo.toml` cannot be read, parsed, or if the folder structure is invalid.
///
pub fn is_valid_steel_project() -> anyhow::Result<bool> {
    let root_toml = fs::read_to_string("./Cargo.toml")?;
    let parsed: Value = root_toml.parse().expect("error reading root toml file");
    let has_steel_dep = parsed
        .get("workspace")
        .and_then(|w| w.get("dependencies"))
        .and_then(|d| d.get("steel"))
        .is_some();

    let has_api_and_program =
        has_directories("./", vec!["api".to_string(), "program".to_string()])?;

    Ok(has_steel_dep && has_api_and_program)
}

/// Retrieves the project name from the `Cargo.toml` file.
///
/// # Returns
/// - `Ok(String)` containing the project name if found.
/// - `Err` if `Cargo.toml` cannot be read, parsed, or `name` field is missing.
pub fn get_project_name() -> anyhow::Result<String> {
    let project_toml = fs::read_to_string("./program/Cargo.toml")?;
    let parsed: Value = project_toml.parse().expect("Could not parse TOML");

    let name = parsed
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(|name| name.as_str())
        .ok_or_else(|| anyhow::anyhow!("Couldn't get package name"))?;

    let name = name.to_string();

    Ok(name)
}

/// Check if project has been built
///
/// # Returns
/// - `Ok(bool)` true if `target` is found.
/// - `Err` if `target` dir isn't found
pub fn is_project_built() -> anyhow::Result<bool> {
    let has_deploy_dir = has_directories("./target", vec!["deploy".to_string()])?;

    Ok(has_deploy_dir)
}

/// Checks if a directory contains subdirectories matching specific names.
///
/// # Arguments
/// - `dir`: The path to the directory to scan.
/// - `dirs`: A list of strings to match against subdirectory names.
///
/// # Returns
/// - `Ok(true)` if the number of matching subdirectories equals `dirs.len()`.
/// - `Ok(false)` otherwise.
/// - `Err` if the directory cannot be read o .
///
/// # FIXME
/// - The `contains` method matches partial words (e.g., "ap" or "prog").
pub fn has_directories(dir: &str, dirs: Vec<String>) -> anyhow::Result<bool> {
    let expected_count = dirs.len();

    let count = fs::read_dir(dir)?
        .filter_map(|d| d.ok())
        .filter(|d| d.path().is_dir())
        .filter(|d| {
            let path = d.path();
            let path_str = path.to_str().unwrap_or("");
            dirs.iter().any(|dir| path_str.contains(dir)) // fixme: `contains` also matches partial words (e.g., "ap" or "prog").
        })
        .count()
        == expected_count;

    Ok(count)
}

/// Replaces the declared program id.
///
/// # Arguments
/// - `new_key`: optional `Keypair`. If `None`, new `Keypair` is generated.
///
/// # Returns
/// - `Ok(())` if program ID is successfully replaced.
/// - `Err` if file cannot be read, written, or if the `declare_id!` macro is not found.
pub fn replace_prog_id(new_key: Option<Keypair>) -> anyhow::Result<()> {
    let new_key = new_key.unwrap_or_else(Keypair::new);
    let lib_rs_path = "./api/src/lib.rs";

    let mut contents = fs::read_to_string(lib_rs_path)?;
    let offset = contents.find("declare_id!").unwrap_or(contents.len());
    let offset_start = offset + 11;
    let formatted_key = format!("(\"{}\");", new_key.pubkey().to_string().as_str());
    contents.replace_range(offset_start.., &formatted_key);

    // write to file
    let mut lib_rs = fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(lib_rs_path)?;
    lib_rs.write_all(contents.as_bytes())?;
    lib_rs.flush()?;

    Ok(())
}
