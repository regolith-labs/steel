mod args;
mod build_project;
mod clean_project;
mod config;
mod new_project;
mod program_keys;
mod test_project;
mod utils;

use args::*;
use build_project::*;
use clap::{command, Parser, Subcommand};
use clean_project::*;
use config::{load_client_and_signer, CommitmentLevel};
use new_project::*;
use program_keys::*;
use test_project::*;

#[derive(Subcommand, Debug)]
enum Command {
    #[command(about = "Create a new Solana program")]
    New(NewArgs),

    #[command(about = "Compile a program and all of its dependencies")]
    Build(BuildArgs),

    #[command(about = "Execute all unit and integration tests")]
    Test(TestArgs),

    #[command(about = "Remove artifacts cargo has generated in the past")]
    Clean(CleanArgs),

    #[command(about = "Program Keypair commands.")]
    Keys(ProgramKeysArgs),
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
        Command::Build(args) => build_project(args),
        Command::Clean(args) => clean_project(args),
        Command::New(args) => new_project(args),
        Command::Test(args) => test_project(args),
        Command::Keys(args) => match args.command {
            KeysSubcommand::List => list_keypair(args),
            KeysSubcommand::New => new_keypair(args),
            KeysSubcommand::Sync => sync_keypair(args),
        },
    }
}
