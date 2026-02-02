use std::{fs::canonicalize, os::unix, path::PathBuf, process::exit};

use serde::{Deserialize, Serialize};
use shellexpand;

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub name: String,
    pub src: PathBuf,
    pub dest: PathBuf,
}

impl Item {
    pub fn from(name: String, src: String, dest: String) -> Self {
        Self {
            name,
            src: PathBuf::from(src),
            dest: PathBuf::from(dest),
        }
    }

    pub fn bash_line(&self) -> String {
        format!(
            "ln -s {} {} #{}",
            self.expanded_absolute_src_path()
                .to_str()
                .expect("Couldn't output path"),
            self.expanded_absolute_dest_path()
                .to_str()
                .expect("Couldn't output path"),
            self.name
        )
    }

    pub fn ps_line(&self) -> String {
        format!(
            "New-Item -ItemType SymbolicLink -Path {} -Target {} #{}",
            self.expanded_absolute_src_path()
                .to_str()
                .expect("Couldn't output path"),
            self.expanded_absolute_dest_path()
                .to_str()
                .expect("Couldn't output path"),
            self.name
        )
    }

    pub fn cmd_line(&self) -> String {
        "Still Testing".to_string()
    }

    pub fn apply(&self) {
        #[cfg(unix)]
        {
            // TODO: check paths
            match unix::fs::symlink(
                &self.expanded_absolute_src_path(),
                &self.expanded_absolute_dest_path(),
            ) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        #[cfg(windows)]
        {
            //TODO: idk maybe implement this sometime
        }
    }

    pub fn expanded_src_path(&self) -> PathBuf {
        // TODO: Better error handling
        match shellexpand::full(&self.src.to_str().expect("Internal error")) {
            Ok(path) => PathBuf::from(path.to_string()),
            Err(e) => {
                eprintln!(
                    "Couldn't expand path {}: {}",
                    &self.src.to_str().expect("Internal error"),
                    e
                );
                exit(1);
            }
        }
    }

    pub fn expanded_dest_path(&self) -> PathBuf {
        // TODO: Better error handling
        match shellexpand::full(&self.dest.to_str().expect("Internal error")) {
            Ok(path) => PathBuf::from(path.to_string()),
            Err(e) => {
                eprintln!(
                    "Couldn't expand path {}: {}",
                    &self.dest.to_str().expect("Internal error"),
                    e
                );
                exit(1);
            }
        }
    }

    pub fn expanded_absolute_src_path(&self) -> PathBuf {
        let path = self.expanded_src_path();
        if path.is_relative() {
            match canonicalize(path) {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("Error parsing source path for {}: {}", self.name, e);
                    exit(1);
                }
            }
        } else {
            path
        }
    }

    pub fn expanded_absolute_dest_path(&self) -> PathBuf {
        let path = self.expanded_dest_path();
        if path.is_relative() {
            match canonicalize(path) {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("Error parsing destination path for {}: {}", self.name, e);
                    exit(1);
                }
            }
        } else {
            path
        }
    }
}
