use std::process::exit;

use anyhow::Error;
use clap::Parser;

use crate::{
    cli::{Cli, Commands},
    commands::{apply, generate, init, show},
};

mod cli;
mod commands;
mod config;
mod constants;
mod item;

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}

fn run() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { skip_git } => init::run(skip_git),
        Commands::Show => show::run(),
        Commands::Gen { format } => generate::run(format),
        Commands::Apply => apply::run(),
    }
}
