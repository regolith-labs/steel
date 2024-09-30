use std::str::FromStr;

use anyhow::anyhow;
use clap::ValueEnum;
use solana_clap_v3_utils::{
    input_validators::normalize_to_url_if_moniker, keypair::signer_from_path,
};
use solana_cli_config::Config;
use solana_sdk::{commitment_config::CommitmentConfig, signer::Signer};

/// Load a signer and a client URL from args, sourced in the following order:
/// 1. Passed in directly to this function.
/// 2. Values specified in the Solana CLI config YAML file.
/// 3. Values in `solana_cli_config::Config::default()`.
pub fn load_client_and_signer(
    client_url: Option<String>,
    commitment_arg: Option<CommitmentLevel>,
    signer_uri: Option<String>,
) -> anyhow::Result<((String, CommitmentConfig), Box<dyn Signer + 'static>)> {
    if let (Some(client_url), Some(signer_uri)) = (&client_url, &signer_uri) {
        let client_url = normalize_to_url_if_moniker(client_url);
        let commitment = commitment_arg.unwrap_or(CommitmentLevel::Confirmed);
        let signer = load_signer(&signer_uri)?;
        Ok(((client_url.clone(), commitment.into()), signer))
    } else {
        let Config {
            keypair_path,
            json_rpc_url,
            commitment,
            ..
        } = load_config().unwrap_or_default();
        let url = client_url.unwrap_or(json_rpc_url);
        let url = normalize_to_url_if_moniker(&url);
        let commitment = commitment_arg.map(Into::into).unwrap_or(
            CommitmentConfig::from_str(&commitment).map_err(|_| {
                anyhow!(
                    "failed to parse commitment string in config file {:?}",
                    commitment
                )
            })?,
        );
        let signer = load_signer(&signer_uri.unwrap_or(keypair_path))?;
        Ok(((url, commitment), signer))
    }
}

/// Load a signer from a URI. This may be a remote wallet, a file, or a seed phrase passed in via command-line.
/// Some URI schemes allow for specifying a derivation path via a query parameter in the form of: `?path=x/y`.
pub fn load_signer(signer_uri: &str) -> anyhow::Result<Box<dyn Signer + 'static>> {
    signer_from_path(
        &clap_v3::ArgMatches::default(),
        &signer_uri,
        "keypair",
        &mut None,
    )
    .map_err(|e| anyhow!("failed to parse signer URI {}", &signer_uri).context(e.to_string()))
}

/// Load the Solana CLI config from its default location.
fn load_config() -> anyhow::Result<Config> {
    let config_file = solana_cli_config::CONFIG_FILE
        .as_ref()
        .ok_or_else(|| anyhow!("unable to get config file path"))?;
    Config::load(&config_file)
        .map_err(|e| anyhow!("failed to load Solana CLI config file {config_file}: {e}"))
}

/// Parsed by Clap, supports better help text and error messages.
#[derive(ValueEnum, Debug, Default, Clone)]
pub enum CommitmentLevel {
    Processed,
    #[default]
    Confirmed,
    Finalized,
}

impl Into<CommitmentConfig> for CommitmentLevel {
    fn into(self) -> CommitmentConfig {
        match self {
            CommitmentLevel::Processed => CommitmentConfig::processed(),
            CommitmentLevel::Confirmed => CommitmentConfig::confirmed(),
            CommitmentLevel::Finalized => CommitmentConfig::finalized(),
        }
    }
}
