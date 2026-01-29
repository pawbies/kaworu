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

    config.items.iter().for_each(|item| item.apply());
}
