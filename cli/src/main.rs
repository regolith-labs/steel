mod args;
mod new_project;
mod utils;

use args::*;
use clap::{command, Parser, Subcommand};
use new_project::*;

#[derive(Subcommand, Debug)]
enum Command {
    #[command(about = "Create a new Solana program")]
    New(NewArgs),
}

#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::New(args) => new_project(args),
    }
}
