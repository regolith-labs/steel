mod args;
mod config;
mod new_project;
mod utils;

use args::*;
use clap::{command, Parser, Subcommand};
use config::{load_client_and_signer, CommitmentLevel};
use new_project::*;

#[derive(Subcommand, Debug)]
enum Command {
    #[command(about = "Create a new Solana program")]
    New(NewArgs),
}

#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Args {
    /// Sets the primary signer on any subcommands that require a signature.
    /// Defaults to the signer in Solana CLI's YAML config file which is
    /// usually located at `~/.config/solana/cli/config.yml`.
    /// This arg is parsed identically to the vanilla Solana CLI and
    /// supports `usb://` and `prompt://` URI schemes as well as filepaths to keypair JSON files.
    #[clap(long, short, env)]
    keypair: Option<String>,
    /// Sets the Solana RPC URL.
    /// Defaults to the `rpc_url` in Solana CLI's YAML config file which is
    /// usually located at `~/.config/solana/cli/config.yml`.
    #[clap(long, short, env = "RPC_URL")]
    url: Option<String>,
    /// Set the default commitment level of any RPC client requests.
    #[clap(long, env)]
    commitment: Option<CommitmentLevel>,
    #[command(subcommand)]
    command: Command,
}

fn main() -> anyhow::Result<()> {
    let Args {
        keypair,
        url,
        commitment,
        command,
    } = Args::parse();
    let (_url, _signer) = load_client_and_signer(url, commitment, keypair)?;
    match command {
        Command::New(args) => new_project(args),
    }
}
