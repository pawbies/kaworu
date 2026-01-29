use std::process::exit;

use crate::config::Config;

pub fn run() {
    let config = match Config::from_file("kaworu.toml".to_string()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    };

    for item in config.items {
        println!(
            "{}: {} -> {}",
            item.name,
            item.src.to_str().unwrap(),
            item.dest.to_str().unwrap()
        );
    }
}
