use solana_sdk::{signature::Keypair, signer::Signer};
use std::fs::{self};

use crate::{
    utils::{get_project_name, is_project_built, is_valid_steel_project},
    ProgramKeysArgs,
};

pub fn list(_args: ProgramKeysArgs) -> anyhow::Result<()> {
    if !is_valid_steel_project()? {
        anyhow::bail!("Not a valid Steel project!");
    }

    if !is_project_built()? {
        anyhow::bail!("Please build project first by running `steel build`!");
    }

    let name = get_project_name()?;

    let formatted = name.replace("-", "_");
    let deploy_name = format!("./target/deploy/{}-keypair.json", formatted);

    // read public key
    let keypair_file = fs::read_to_string(deploy_name)?;

    let keypair_bytes: Vec<u8> = serde_json::from_str(&keypair_file)?;
    let keypair = Keypair::from_bytes(&keypair_bytes)?;

    println!("{name}: {:#?}", keypair.pubkey());

    Ok(())
}
