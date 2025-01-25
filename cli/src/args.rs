use clap::{arg, Parser};

#[derive(Parser, Debug)]
pub struct NewArgs {
    #[arg(value_name = "NAME", help = "The name of the program")]
    pub name: Option<String>,
}

#[derive(Parser, Debug)]
pub struct BuildArgs {}

#[derive(Parser, Debug)]
pub struct TestArgs {}

#[derive(Parser, Debug)]
pub struct CleanArgs {}

#[derive(Parser, Debug)]
pub struct ProgramKeysArgs {
    #[command(subcommand)]
    pub command: KeysSubcommand,
}

#[derive(Parser, Debug)]
pub enum KeysSubcommand {
    #[command(about = "List program keypair")]
    List,

    #[command(about = "Replace existing program keypair with new one")]
    New,

    #[command(about = "Sync declared program id to deploy program keypair")]
    Sync,
}
