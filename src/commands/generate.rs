use crate::cli::ScriptFormat::{self, *};

pub fn run(format: ScriptFormat) {
    match format {
        Bash => println!("Bash"),
        Powershell => println!("Powershell"),
        CMD => println!("CMD"),
    }
}
