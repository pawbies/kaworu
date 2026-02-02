use std::{
    error::Error,
    fs::canonicalize,
    os::unix,
    path::{PathBuf, absolute},
    process::exit,
};

use serde::{Deserialize, Serialize};
use shellexpand;

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub name: String,
    pub src: PathBuf,
    pub dest: PathBuf,
}

impl Item {
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
        match expanded_path(&self.src) {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Couldn't expand source path for {}: {}", self.name, e);
                exit(1);
            }
        }
    }

    pub fn expanded_dest_path(&self) -> PathBuf {
        match expanded_path(&self.dest) {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Couldn't expand destination path for {}: {}", self.name, e);
                exit(1);
            }
        }
    }

    pub fn expanded_absolute_src_path(&self) -> PathBuf {
        match absolute_path(&self.expanded_src_path(), true) {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Error parsing source path for {}: {}", self.name, e);
                exit(1);
            }
        }
    }

    pub fn expanded_absolute_dest_path(&self) -> PathBuf {
        match absolute_path(&self.expanded_dest_path(), false) {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Error parsing destination path for {}: {}", self.name, e);
                exit(1);
            }
        }
    }
}

fn expanded_path(path: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    Ok(shellexpand::full(path.to_str().expect("Internal error"))?
        .to_string()
        .into())
}

fn absolute_path(path: &PathBuf, require_existence: bool) -> std::io::Result<PathBuf> {
    if path.is_relative() {
        if require_existence {
            canonicalize(path)
        } else {
            absolute(path)
        }
    } else {
        Ok(path.clone())
    }
}
