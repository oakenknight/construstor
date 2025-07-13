//! # Construstor - Smart Contract Constructor and Initialize Function Analyzer
//!
//! A library for analyzing Solidity smart contracts to detect zero address validation
//! patterns in constructors and initialize functions.

pub mod analyzer;
pub mod cli;
pub mod printer;
pub mod types;

use std::error::Error;

pub use analyzer::ConstructorAnalyzer;
pub use cli::CliConfig;
use colored::*;
pub use printer::ResultPrinter;
pub use types::*;

/// Main application logic
pub fn run(config: CliConfig) -> Result<(), Box<dyn Error>> {
    let analyzer = ConstructorAnalyzer::new()?;

    match analyzer.analyze_path(&config.input_path) {
        Ok(results) => {
            if config.json_output {
                println!("{}", serde_json::to_string_pretty(&results)?);
            } else if config.summary_only {
                ResultPrinter::print_summary(&results);
            } else {
                ResultPrinter::print_results(&results);
                if !results.is_empty() {
                    ResultPrinter::print_summary(&results);
                }
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("{}: {}", "Error".red().bold(), e);
            Err(Box::new(e))
        }
    }
}
