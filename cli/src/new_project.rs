use std::{fs, io, path::Path};

use git2::Repository;

use crate::{
    utils::{to_camel_case, to_lib_case, to_type_case},
    NewArgs,
};

pub fn new_project(args: NewArgs) {
    let project_name = args.name.to_ascii_lowercase();
    let base_path = Path::new(&project_name);
    stub_workspace(base_path, &project_name).unwrap();
    stub_api(base_path, &project_name).unwrap();
    stub_program(base_path, &project_name).unwrap();
}

fn stub_workspace(base_path: &Path, project_name: &String) -> io::Result<()> {
    // Create folder
    fs::create_dir_all(&base_path).expect("Failed to create workspace directory");

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
    fs::create_dir_all(&api_src_state_path).expect("Failed to create api directory");

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

    // Create folders
    fs::create_dir_all(&program_src_path).expect("Failed to create program directory");

    // Load templates
    const PROGRAM_CARGO_TOML: &str = include_str!("template/program_cargo_toml");
    const PROGRAM_SRC_LIB_RS: &str = include_str!("template/program_src_lib_rs");
    const PROGRAM_SRC_ADD_RS: &str = include_str!("template/program_src_add_rs");
    const PROGRAM_SRC_INITIALIZE_RS: &str = include_str!("template/program_src_initialize_rs");

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
