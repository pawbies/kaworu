use std::{fs::write, process::exit};

use crate::{
    cli::ScriptFormat::{self, *},
    config::Config,
    constants::DEFAULT_CONFIG_NAME,
};

pub fn run(format: ScriptFormat) {
    let config = match Config::from_file(DEFAULT_CONFIG_NAME.to_string()) {
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
