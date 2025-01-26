use solana_sdk::{signature::Keypair, signer::Signer};
use std::{
    fs::{self},
    io::Write,
};

use crate::{
    utils::{get_project_name, is_project_built, is_valid_steel_project, replace_prog_id},
    ProgramKeysArgs,
};

pub fn list_keypair(_args: ProgramKeysArgs) -> anyhow::Result<()> {
    if !is_valid_steel_project()? {
        anyhow::bail!("Not a valid Steel project!");
    }

    if !is_project_built()? {
        anyhow::bail!("Please build project first by running `steel build`!");
    }

    let name = get_project_name()?;
    let formatted = name.replace("-", "_");
    let deploy_kp_path = format!("./target/deploy/{}-keypair.json", formatted);

    // read public key
    let keypair_file = fs::read_to_string(deploy_kp_path)?;

    let keypair_bytes: Vec<u8> = serde_json::from_str(&keypair_file)?;
    let keypair = Keypair::from_bytes(&keypair_bytes)?;

    println!("{name}: {:#?}", keypair.pubkey());

    Ok(())
}

pub fn new_keypair(_args: ProgramKeysArgs) -> anyhow::Result<()> {
    if !is_valid_steel_project()? {
        anyhow::bail!("Not a valid Steel project!");
    }

    if !is_project_built()? {
        anyhow::bail!("Please build project first by running `steel build`!");
    }

    // replace declare_id! address
    let new_key = Keypair::new();
    replace_prog_id(new_key.insecure_clone().into())?;

    // replace keypair in deploy
    let name = get_project_name()?;
    let formatted = name.replace("-", "_");
    let deploy_kp_path = format!("./target/deploy/{}-keypair.json", formatted);

    let mut lib_rs = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(deploy_kp_path)?;
    lib_rs.write_all(&format!("{:?}", new_key.to_bytes()).as_bytes())?;
    lib_rs.flush()?;

    Ok(())
}
