use std::{
    fs::{read_to_string, write},
    process::exit,
};

use crate::{
    cli::ScriptFormat::{self, *},
    config::Config,
};

pub fn run(format: ScriptFormat) {
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

    let script: String = match format {
        Bash => config
            .items
            .iter()
            .map(|item| item.bash_line() + "\n")
            .collect(),
        Powershell => config
            .items
            .iter()
            .map(|item| item.ps_line() + "\n")
            .collect(),
        CMD => config
            .items
            .iter()
            .map(|item| item.cmd_line() + "\n")
            .collect(),
    };
    let file_name = match format {
        Bash => "kaworu.sh",
        Powershell => "kaworu.ps",
        CMD => "kaworu.cmd",
    };
    match write(file_name, script) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}
