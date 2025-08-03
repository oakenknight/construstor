use std::process;

use construstor::{CliConfig, run};

fn main() {
    let config = match CliConfig::from_args() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error parsing arguments: {e}");
            process::exit(1);
        }
    };

    if let Err(e) = run(config) {
        eprintln!("Fatal error: {e}");
        process::exit(1);
    }
}
