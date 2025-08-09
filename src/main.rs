use tera::Tera;

struct Karka {
}

impl Karka {
}

fn main() {
    let tera = match Tera::new("src/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };
}
