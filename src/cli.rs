use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    //// Build site
    Build,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
