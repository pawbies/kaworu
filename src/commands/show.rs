use std::process::exit;

use crate::{config::Config, constants::DEFAULT_CONFIG_NAME};

pub fn run() {
    let config = match Config::from_file(DEFAULT_CONFIG_NAME.to_string()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    };

    // TODO: Error handling
    for item in config.items {
        println!(
            "{}: {} ({}) -> {} ({})",
            item.name,
            item.expanded_absolute_src_path().to_str().unwrap(),
            item.src.to_str().unwrap(),
            item.expanded_absolute_dest_path().to_str().unwrap(),
            item.dest.to_str().unwrap()
        );
    }
}
