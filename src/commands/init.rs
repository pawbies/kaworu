use std::{fs, io::ErrorKind, process::Command};

use crate::{config::Config, constants::DEFAULT_CONFIG_NAME};

pub fn run(skip_git: bool) {
    match fs::exists(DEFAULT_CONFIG_NAME) {
        Ok(b) => {
            if b {
                println!("{} already in directory", DEFAULT_CONFIG_NAME);
            } else {
                init_kaworu_repo();
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    if !skip_git {
        init_git_repo();
    }
}

fn init_kaworu_repo() {
    let sample_toml = Config::default().to_toml();
    match fs::write(DEFAULT_CONFIG_NAME, &sample_toml) {
        Ok(_) => println!("Wrote {}", DEFAULT_CONFIG_NAME),
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => eprintln!("Error: permission denied"),
            ErrorKind::AlreadyExists => eprintln!("Error: {} already exists", DEFAULT_CONFIG_NAME),
            _ => eprintln!("Error: {e}"),
        },
    }
}

fn init_git_repo() {
    let status = Command::new("git")
        .arg("init")
        .status()
        .expect("Failed to run git");
    if status.success() {
        println!("Git repo initialized");
    } else {
        eprintln!("Couldn't inizialize git repo");
    }
}
