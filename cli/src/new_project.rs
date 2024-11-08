use std::{fs, io, path::Path};

use colored::*;
use git2::{FetchOptions, RemoteCallbacks, Repository};

use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::{
    utils::{prompt, to_camel_case, to_lib_case, to_type_case},
    NewArgs,
};

pub struct TemplateHandler;

impl TemplateHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn clone_and_process(
        &self,
        url: &str,
        target_dir: &Path,
        project_name: &str,
    ) -> Result<()> {
        if target_dir.exists() {
            return Err(anyhow::anyhow!(
                "Directory '{}' already exists",
                target_dir.display()
            ));
        }

        fs::create_dir_all(target_dir)
            .with_context(|| format!("Failed to create directory: {}", target_dir.display()))?;

        let temp_dir = tempfile::TempDir::new().context("Failed to create temporary directory")?;

        let mut callbacks = RemoteCallbacks::new();
        let mut last_progress = 0;
        callbacks.transfer_progress(|progress| {
            let current = progress.received_objects();
            let total = progress.total_objects();
            if current != last_progress && current == total {
                println!("Template downloaded successfully!");
            }
            last_progress = current;
            true
        });

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);

        println!("Downloading template...");
        builder
            .clone(url, temp_dir.path())
            .with_context(|| format!("Failed to clone template repository from {}", url))?;

        self.copy_and_process_directory(temp_dir.path(), target_dir, project_name)?;
        Repository::init(target_dir)?;

        Ok(())
    }

    fn copy_and_process_directory(
        &self,
        source: &Path,
        target: &Path,
        project_name: &str,
    ) -> Result<()> {
        for entry in WalkDir::new(source).min_depth(1) {
            let entry = entry?;
            let path = entry.path();

            if path.components().any(|c| c.as_os_str() == ".git") {
                continue;
            }

            let relative_path = path.strip_prefix(source)?;
            let target_path = target.join(relative_path);

            if entry.file_type().is_dir() {
                fs::create_dir_all(&target_path).with_context(|| {
                    format!("Failed to create directory: {}", target_path.display())
                })?;
            } else {
                if let Some(parent) = target_path.parent() {
                    fs::create_dir_all(parent).with_context(|| {
                        format!("Failed to create parent directory: {}", parent.display())
                    })?;
                }

                let content = fs::read_to_string(path)
                    .with_context(|| format!("Failed to read file: {}", path.display()))?;

                let processed_content = content
                    .replace("{name_lowercase}", &project_name.to_ascii_lowercase())
                    .replace("{name_uppercase}", &project_name.to_ascii_uppercase())
                    .replace("{name_camelcase}", &to_camel_case(&project_name))
                    .replace("{name_typecase}", &to_type_case(&project_name))
                    .replace("{name_libcase}", &to_lib_case(&project_name));

                fs::write(&target_path, processed_content)
                    .with_context(|| format!("Failed to write file: {}", target_path.display()))?;
            }
        }
        Ok(())
    }
}

pub fn new_project(args: NewArgs) -> Result<()> {
    // Get project name
    let project_name = if let Some(name) = args.name {
        name.to_ascii_lowercase()
    } else {
        let name = prompt("Please provide a project name:");
        if name.is_empty() {
            panic!("{}: Project name cannot be empty.", "ERROR".bold().red());
        }
        name.to_ascii_lowercase()
    };

    // TODO Get names of accounts
    // TODO Get names of instruction
    // TODO For each account name:
    //      - Stub an enum value in api/src/state/mod.rs
    //      - Stub a file in api/src/state/
    // TODO For each instruction name:
    //      - Stub an enum value in api/src/instruction.rs
    //      - Stub an file in program/src/
    // TODO Get metadata:
    //      - Homepage
    //      - Repository
    //      - Generate docs link

    let base_path = Path::new(&project_name);

    if let Some(template_url) = args.template_url {
        let handler = TemplateHandler::new();
        handler.clone_and_process(&template_url, base_path, &project_name)?;
        println!("✨ Project '{}' created successfully!", project_name);
    } else {
        stub_workspace(base_path, &project_name)?;
        stub_api(base_path, &project_name)?;
        stub_program(base_path, &project_name)?;
    }

    Ok(())
}

fn stub_workspace(base_path: &Path, project_name: &String) -> io::Result<()> {
    // Create folder
    fs::create_dir_all(&base_path)?;

    // Load templates
    const CARGO_TOML: &str = include_str!("template/cargo_toml");
    const GITIGNORE: &str = include_str!("template/gitignore");
    const README_MD: &str = include_str!("template/readme_md");

    // Stub files
    stub_file(CARGO_TOML, &base_path.join("Cargo.toml"), project_name)?;
    stub_file(GITIGNORE, &base_path.join(".gitignore"), project_name)?;
    stub_file(README_MD, &base_path.join("README.md"), project_name)?;

    // Initialize git
    Repository::init(base_path).unwrap();

    Ok(())
}

fn stub_api(base_path: &Path, project_name: &String) -> io::Result<()> {
    // Derive paths
    let api_path = base_path.join("api");
    let api_src_path = api_path.join("src");
    let api_src_state_path = api_src_path.join("state");

    // Create folders
    fs::create_dir_all(&api_src_state_path)?;

    // Load templates
    const API_CARGO_TOML: &str = include_str!("template/api_cargo_toml");
    const API_SRC_LIB_RS: &str = include_str!("template/api_src_lib_rs");
    const API_SRC_CONSTS_RS: &str = include_str!("template/api_src_consts_rs");
    const API_SRC_ERROR_RS: &str = include_str!("template/api_src_error_rs");
    const API_SRC_INSTRUCTION_RS: &str = include_str!("template/api_src_instruction_rs");
    const API_SRC_SDK_RS: &str = include_str!("template/api_src_sdk_rs");
    const API_SRC_STATE_MOD_RS: &str = include_str!("template/api_src_state_mod_rs");
    const API_SRC_STATE_COUNTER_RS: &str = include_str!("template/api_src_state_counter_rs");

    // Stub files
    stub_file(API_CARGO_TOML, &api_path.join("Cargo.toml"), project_name)?;
    stub_file(API_SRC_LIB_RS, &api_src_path.join("lib.rs"), project_name)?;
    stub_file(
        API_SRC_CONSTS_RS,
        &api_src_path.join("consts.rs"),
        project_name,
    )?;
    stub_file(
        API_SRC_ERROR_RS,
        &api_src_path.join("error.rs"),
        project_name,
    )?;
    stub_file(
        API_SRC_INSTRUCTION_RS,
        &api_src_path.join("instruction.rs"),
        project_name,
    )?;
    stub_file(API_SRC_SDK_RS, &api_src_path.join("sdk.rs"), project_name)?;
    stub_file(
        API_SRC_STATE_MOD_RS,
        &api_src_state_path.join("mod.rs"),
        project_name,
    )?;
    stub_file(
        API_SRC_STATE_COUNTER_RS,
        &api_src_state_path.join("counter.rs"),
        project_name,
    )?;

    Ok(())
}

fn stub_program(base_path: &Path, project_name: &String) -> io::Result<()> {
    // Derive paths
    let program_path = base_path.join("program");
    let program_src_path = program_path.join("src");
    let program_tests_path = program_path.join("tests");

    // Create folders
    fs::create_dir_all(&program_src_path)?;
    fs::create_dir_all(&program_tests_path)?;

    // Load templates
    const PROGRAM_CARGO_TOML: &str = include_str!("template/program_cargo_toml");
    const PROGRAM_SRC_LIB_RS: &str = include_str!("template/program_src_lib_rs");
    const PROGRAM_SRC_ADD_RS: &str = include_str!("template/program_src_add_rs");
    const PROGRAM_SRC_INITIALIZE_RS: &str = include_str!("template/program_src_initialize_rs");
    const PROGRAM_TESTS_TEST_RS: &str = include_str!("template/program_tests_test_rs");

    // Stub files
    stub_file(
        PROGRAM_CARGO_TOML,
        &program_path.join("Cargo.toml"),
        project_name,
    )?;
    stub_file(
        PROGRAM_SRC_LIB_RS,
        &program_src_path.join("lib.rs"),
        project_name,
    )?;
    stub_file(
        PROGRAM_SRC_ADD_RS,
        &program_src_path.join("add.rs"),
        project_name,
    )?;
    stub_file(
        PROGRAM_SRC_INITIALIZE_RS,
        &program_src_path.join("initialize.rs"),
        project_name,
    )?;
    stub_file(
        PROGRAM_TESTS_TEST_RS,
        &program_tests_path.join("test.rs"),
        project_name,
    )?;

    Ok(())
}

fn stub_file(template: &str, path: &Path, project_name: &String) -> io::Result<()> {
    let content = template
        .replace("{name_lowercase}", &project_name.to_ascii_lowercase())
        .replace("{name_uppercase}", &project_name.to_ascii_uppercase())
        .replace("{name_camelcase}", &to_camel_case(&project_name))
        .replace("{name_typecase}", &to_type_case(&project_name))
        .replace("{name_libcase}", &to_lib_case(&project_name));
    fs::write(path, content)?;
    Ok(())
}
