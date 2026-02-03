use std::fs::write;

use anyhow::{Error, anyhow};

use crate::{
    cli::ScriptFormat::{self, *},
    config::Config,
    constants::DEFAULT_CONFIG_NAME,
};

pub fn run(format: ScriptFormat) -> Result<(), Error> {
    let config = Config::from_file(DEFAULT_CONFIG_NAME.to_string())?;

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
        Ok(_) => {
            println!("Wrote {}", file_name);
            Ok(())
        }
        Err(e) => Err(anyhow!(e)),
    }
}
