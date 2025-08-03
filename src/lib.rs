//! # Construstor - Smart Contract Constructor, Initialize Function, and Address Parameter Analyzer
//!
//! A library for analyzing Solidity smart contracts to detect zero address validation
//! patterns in constructors, initialize functions, and all functions with address parameters.

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

    match analyzer.analyze_path(&config.input_path, config.all_functions) {
        Ok(results) => {
            if config.json_output {
                let json_results: Vec<AnalysisResultJson> =
                    results.iter().map(|r| r.into()).collect();
                println!("{}", serde_json::to_string_pretty(&json_results)?);
            } else if config.summary_only {
                ResultPrinter::print_summary(&results);
            } else {
                ResultPrinter::print_results(&results);
                if !results.is_empty() {
                    ResultPrinter::print_summary(&results);
                }
                println!("Analysis complete!");
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("{}: {}", "Error".red().bold(), e);
            Err(Box::new(e))
        }
    }
}
