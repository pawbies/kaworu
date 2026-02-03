use std::{fs, process::Command};

use anyhow::{Error, anyhow};

use crate::{config::Config, constants::DEFAULT_CONFIG_NAME};

pub fn run(skip_git: bool) -> Result<(), Error> {
    match fs::exists(DEFAULT_CONFIG_NAME) {
        Ok(file_exists) => {
            if file_exists {
                println!("{} already in directory", DEFAULT_CONFIG_NAME);
            } else {
                init_kaworu_repo()?;
            }
        }
        Err(e) => return Err(anyhow!(e)),
    }
    if !skip_git {
        init_git_repo()?;
    }
    Ok(())
}

fn init_kaworu_repo() -> Result<(), Error> {
    let sample_toml = Config::default().to_toml();
    match fs::write(DEFAULT_CONFIG_NAME, &sample_toml) {
        Ok(_) => println!("Wrote {}", DEFAULT_CONFIG_NAME),
        Err(e) => return Err(anyhow!(e)),
    }

    Ok(())
}

fn init_git_repo() -> Result<(), Error> {
    let status = Command::new("git")
        .arg("init")
        .status()
        .expect("Failed to run git");
    if status.success() {
        println!("Git repo initialized");
        Ok(())
    } else {
        Err(anyhow!("Couldn't initialize git repo"))
    }
}
