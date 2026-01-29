use std::{error::Error, fs::read_to_string, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub items: Vec<Item>,
}

impl Default for Config {
    fn default() -> Self {
        let items = vec![Item::from(
            "kaworu".to_string(),
            PathBuf::from("kaworu.toml"),
            PathBuf::from("kaworu_link.toml"),
        )];
        Self { items: items }
    }
}

impl Config {
    pub fn to_toml(&self) -> String {
        toml::to_string_pretty(&self).expect("Couldn't convert config struct to toml")
    }
    pub fn from_toml(text: String) -> Result<Self, Box<dyn Error>> {
        Ok(toml::from_str::<Config>(text.as_str())?)
    }

    pub fn from_file(file: String) -> Result<Self, Box<dyn Error>> {
        let file_contents = read_to_string(file)?;
        let config = Config::from_toml(file_contents)?;

        Ok(config)
    }
}
