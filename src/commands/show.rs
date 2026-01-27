use std::{fs::read_to_string, process::exit};

use crate::config::Config;

pub fn run() {
    let file_contents = read_to_string("kaworu.toml").unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        exit(1);
    });

    let config = match Config::from_toml(file_contents) {
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
