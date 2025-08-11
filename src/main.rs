mod cli;
mod config;
mod error;
mod karka;

fn main() {
    env_logger::init();
    log::info!(
        "Starting {} {}...",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let cli = cli::parse_args();

    match cli.command {
        cli::Command::Build => karka::build(cli),
    }
}
