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
}
