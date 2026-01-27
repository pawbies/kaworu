use std::{fs, io::ErrorKind, process::Command};

use crate::config::Config;

pub fn run(skip_git: bool) {
    match fs::exists("kaworu.toml") {
        Ok(b) => {
            if b {
                println!("kaworu.toml already in directory")
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
    match fs::write("kaworu.toml", &sample_toml) {
        Ok(_) => println!("Wrote kaworu.toml"),
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => eprintln!("Error: permission denied"),
            ErrorKind::AlreadyExists => eprintln!("Error: kaworu.toml already exists"),
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
