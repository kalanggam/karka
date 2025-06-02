use std::default;
use tera::Tera;

struct Karka {
    dirs: Paths,
    //taxonomies: type
    //plugins: type
}

impl Karka {
    fn add_taxonomy(&self) {
        todo!();
    }

    fn add_plugin(&self, plugin: Plugin) {
        todo!();
    }

    fn generate(&self) {
        todo!();
    }
}

impl Default for Karka {
    fn default() -> Self {
        Karka {
            dirs: Paths::default(),
        }
    }
}

struct Paths {
    src: String,    // default "/src"
    site: String,   // default "/_site"
    layout: String, // default "/src/_layouts"
    component: String, // default "/src/_components"
                    //asset: String,
                    //style: String,
                    //data: String,
}

impl Default for Paths {
    fn default() -> Self {
        Paths {
            src: String::default(),
            site: String::default(),
            layout: String::default(),
            component: String::default(),
        }
    }
}

struct Plugin {}

fn main() {
    println!("Hello, world!");
    Karka::default().generate();
    let tera = match Tera::new("src/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
}
