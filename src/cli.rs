//! CLI module for handling command-line arguments

use clap::{Arg, Command};
use std::error::Error;

/// CLI configuration
#[derive(Debug)]
pub struct CliConfig {
    pub input_path: String,
    pub summary_only: bool,
    pub json_output: bool,
}

impl CliConfig {
    /// Parse command line arguments
    pub fn from_args() -> Result<Self, Box<dyn Error>> {
        let matches = Command::new("construstor")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about("Analyze Solidity smart contracts for zero address validation patterns")
            .arg(
                Arg::new("input")
                    .help("Path to Solidity file or directory to analyze")
                    .required(false)
                    .index(1),
            )
            .arg(
                Arg::new("summary")
                    .short('s')
                    .long("summary")
                    .help("Show only summary statistics")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("json")
                    .short('j')
                    .long("json")
                    .help("Output results in JSON format")
                    .action(clap::ArgAction::SetTrue),
            )
            .get_matches();

        let input_path = if let Some(path) = matches.get_one::<String>("input") {
            path.clone()
        } else {
            // If no path provided, prompt user
            use dialoguer::Input;
            Input::new()
                .with_prompt("What is the path to the file or folder?")
                .interact_text()?
        };

        Ok(CliConfig {
            input_path,
            summary_only: matches.get_flag("summary"),
            json_output: matches.get_flag("json"),
        })
    }
}
