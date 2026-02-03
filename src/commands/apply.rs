use anyhow::Error;

use crate::{config::Config, constants::DEFAULT_CONFIG_NAME};

pub fn run() -> Result<(), Error> {
    let config = Config::from_file(DEFAULT_CONFIG_NAME.to_string())?;

    config.items.iter().for_each(|item| item.apply());

    Ok(())
}
