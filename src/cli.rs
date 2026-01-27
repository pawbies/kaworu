use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "kaworu")]
#[command(about = "Config file manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new kaworu repo
    Init {
        /// Skip creating a git repo
        #[arg(short, long, default_value = "false")]
        skip_git: bool,
    },
    /// Show the changes that would apply
    Show,
    /// Generate a shell script to apply the changes
    Gen {
        #[arg(default_value = "bash")]
        format: ScriptFormat,
    },
    /// Apply changes
    Apply,
}

#[derive(Clone, ValueEnum)]
pub enum ScriptFormat {
    Bash,
    Powershell,
    CMD,
}
