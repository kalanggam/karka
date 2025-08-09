mod cli;
mod config;
mod error;
mod karka;

use tera::Tera;

use crate::cli::parse_args;

fn main() {
    let tera = match Tera::new("src/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };
}
