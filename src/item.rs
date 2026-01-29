use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub name: String,
    pub src: PathBuf,
    pub dest: PathBuf,
}

impl Item {
    pub fn from(name: String, src: PathBuf, dest: PathBuf) -> Self {
        Self { name, src, dest }
    }

    pub fn bash_line(&self) -> String {
        format!(
            "ln -s {} {} #{}",
            self.src.to_str().expect("Couldn't output path"),
            self.dest.to_str().expect("Couldn't output path"),
            self.name
        )
    }

    pub fn ps_line(&self) -> String {
        format!(
            "New-Item -ItemType SymbolicLink -Path {} -Target {} #{}",
            self.src.to_str().expect("Couldn't output path"),
            self.dest.to_str().expect("Couldn't output path"),
            self.name
        )
    }

    pub fn cmd_line(&self) -> String {
        "Still Testing".to_string()
    }

    pub fn apply(&self) {
        #[cfg(unix)]
        {
            match std::os::unix::fs::symlink(&self.src, &self.dest) {
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
}
