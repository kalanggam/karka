use tera::Tera;

use crate::config::Config;

pub struct Karka {
    config: Config,
}

pub fn build() {
    log::info!("Building site...");

    let tera = match Tera::new("src/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            log::error!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };
}