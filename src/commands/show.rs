use anyhow::{Error, anyhow};

use crate::{config::Config, constants::DEFAULT_CONFIG_NAME};

pub fn run() -> Result<(), Error> {
    let config = Config::from_file(DEFAULT_CONFIG_NAME.to_string())?;

    for item in config.items {
        let original_source = item.src.to_str().ok_or(anyhow!("Couldn't process path"))?;
        let original_destination = item.src.to_str().ok_or(anyhow!("Couldn't process path"))?;

        let processed_source = item.expanded_absolute_src_path();
        let processed_destination = item.expanded_absolute_dest_path();

        let processed_source_str = processed_source
            .to_str()
            .ok_or(anyhow!("Couldn't process path"))?;
        let processed_destination_str = processed_destination
            .to_str()
            .ok_or(anyhow!("Couldn't process path"))?;

        println!(
            "{}: {} ({}) -> {} ({})",
            item.name,
            processed_source_str,
            original_source,
            processed_destination_str,
            original_destination
        );
    }
    Ok(())
}
