use tera::{Context, Tera};

use crate::config::Config;

pub struct Karka {
    config: Config,
    context: Context,
    tera: Tera,
}

impl Karka {
    pub fn build(cli: Cli) -> Result<Self, tera::Error> {
        match Tera::new("templates/**/*") {
            Ok(t) => Ok(Karka {
                config: Config::default(),
                context: Context::new(),
                tera: t,
            }),
            Err(e) => {
                log::error!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
        }
    }
}

pub fn build(cli: Cli) {
    log::info!("Building site...");

    let karka = Karka::build(cli)?;
}
