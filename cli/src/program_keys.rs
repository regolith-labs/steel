use solana_signer::Signer;
use solana_keypair::Keypair;
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

    let project_name = get_project_name()?;
    let formatted = project_name.replace("-", "_");
    let deploy_kp_path = format!("./target/deploy/{}-keypair.json", formatted);

    // read public key
    let keypair_file = fs::read_to_string(deploy_kp_path)?;

    let keypair_bytes: Vec<u8> = serde_json::from_str(&keypair_file)?;
    let keypair = Keypair::from_bytes(&keypair_bytes)?;

    println!("{project_name}: {}", keypair.pubkey().to_string());

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
    let project_name = get_project_name()?;
    let formatted = project_name.replace("-", "_");
    let deploy_kp_path = format!("./target/deploy/{}-keypair.json", formatted);

    let mut lib_rs = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(deploy_kp_path)?;
    lib_rs.write_all(&format!("{:?}", new_key.to_bytes()).as_bytes())?;
    lib_rs.flush()?;

    println!("{project_name}: {}", new_key.pubkey().to_string());

    Ok(())
}

pub fn sync_keypair(_args: ProgramKeysArgs) -> anyhow::Result<()> {
    if !is_valid_steel_project()? {
        anyhow::bail!("Not a valid Steel project!");
    }

    if !is_project_built()? {
        anyhow::bail!("Please build project first by running `steel build`!");
    }

    // read keypair file
    let project_name = get_project_name()?;
    let formatted = project_name.replace("-", "_");
    let deploy_kp_path = format!("./target/deploy/{}-keypair.json", formatted);
    let deploy_kp_contents = fs::read_to_string(deploy_kp_path.clone())?;
    let keypair_bytes: Vec<u8> = serde_json::from_str(&deploy_kp_contents)?;
    let public_key = Keypair::from_bytes(&keypair_bytes)?.pubkey();

    // check if it matches lib.rs
    let lib_rs_contents = fs::read_to_string("./api/src/lib.rs")?;
    let found = lib_rs_contents.find(public_key.to_string().as_str());
    if found.is_some() {
        println!("program keys already synced: {}", public_key.to_string());
        return Ok(());
    }

    // update with this keypair in lib.rs
    let new_key = Keypair::new();
    replace_prog_id(new_key.insecure_clone().into())?;
    let mut lib_rs = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(deploy_kp_path)?;
    lib_rs.write_all(&format!("{:?}", new_key.to_bytes()).as_bytes())?;
    lib_rs.flush()?;
    println!("program keys synced to: {}", new_key.pubkey().to_string());

    Ok(())
}
