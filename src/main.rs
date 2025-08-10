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

    let settings = cli::parse_args();

    match settings.command {
        cli::Command::Build => karka::build(),
    }
}
