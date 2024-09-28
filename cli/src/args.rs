use clap::{arg, Parser};

#[derive(Parser, Debug)]
pub struct NewArgs {
    #[arg(value_name = "NAME", help = "The name of the program")]
    pub name: String,
}
