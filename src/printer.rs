//! Pretty printing functionality for analysis results

use colored::*;

use crate::types::{AnalysisResult, FunctionType, ValidationType};

/// Pretty printer for analysis results
pub struct ResultPrinter;

impl ResultPrinter {
    /// Prints analysis results with colored output
    pub fn print_results(results: &[AnalysisResult]) {
        if results.is_empty() {
            println!("{}", "No functions with address parameters found.".yellow());
            return;
        }

        for result in results {
            Self::print_single_result(result);
        }
    }

    /// Prints a single analysis result
    fn print_single_result(result: &AnalysisResult) {
        let function_name = match &result.function_type {
            FunctionType::Constructor => "Constructor".green(),
            FunctionType::Initialize => "Initialize function".cyan(),
            FunctionType::Regular(name) => format!("Function '{}'", name).magenta(),
        };

        println!("{} in {}:", function_name, result.file_name);

        if result.address_arguments.is_empty() {
            println!("{}", "ℹ️  No address arguments found".blue());
        } else {
            let formatted_args: Vec<String> = result
                .address_arguments
                .iter()
                .map(|(arg_type, arg_name)| format!("{} {}", arg_type, arg_name))
                .collect();

            println!(
                "{}",
                format!(
                    "📋 Found {} address argument(s): {}",
                    result.address_arguments.len(),
                    formatted_args.join(", ")
                )
                .blue()
            );

            if !result.validation_types.is_empty() {
                println!("{}", "✅ Zero address validation found:".green());

                if result
                    .validation_types
                    .contains(&ValidationType::EqualityCheck)
                {
                    println!("  {} Direct address(0) comparison", "•".green());
                }

                if result
                    .validation_types
                    .contains(&ValidationType::RequireStatement)
                {
                    println!(
                        "  {} require() statement with zero address check",
                        "•".green()
                    );
                }

                for var in &result.validated_variables {
                    println!("    {} Checking variable: {}", "→".blue(), var.yellow());
                }
            }

            if !result.missing_validations.is_empty() {
                println!("{}", "❌ Missing zero address validation for:".red());
                for missing_arg in &result.missing_validations {
                    println!("    {} Argument: {}", "⚠️".red(), missing_arg.yellow());
                }
            } else if !result.address_arguments.is_empty() && !result.validation_types.is_empty() {
                println!(
                    "{}",
                    "✅ All address arguments are validated!".green().bold()
                );
            } else if !result.address_arguments.is_empty() {
                println!(
                    "{}",
                    "❌ No zero address validation detected for any argument".red()
                );
            }
        }

        println!("{}", format!("Arguments: {}", result.arguments).yellow());
        println!("{}", "Code:".blue());
        // Print the code with proper indentation
        for line in result.code.lines() {
            println!("  {}", line.blue());
        }
        println!("{}", "=".repeat(50));
    }

    /// Prints a summary of all results
    pub fn print_summary(results: &[AnalysisResult]) {
        if results.is_empty() {
            return;
        }

        let total_functions = results.len();
        let functions_with_address_args: Vec<_> = results
            .iter()
            .filter(|r| !r.address_arguments.is_empty())
            .collect();

        let fully_validated = functions_with_address_args
            .iter()
            .filter(|r| r.missing_validations.is_empty())
            .count();

        let partially_validated = functions_with_address_args
            .iter()
            .filter(|r| !r.missing_validations.is_empty() && !r.validated_variables.is_empty())
            .count();

        let unvalidated = functions_with_address_args
            .iter()
            .filter(|r| r.validated_variables.is_empty())
            .count();

        println!("\n{}", "📊 Analysis Summary:".bold().blue());
        println!(
            "  Total functions analyzed: {}",
            total_functions.to_string().yellow()
        );
        println!(
            "  Functions with address arguments: {}",
            functions_with_address_args.len().to_string().yellow()
        );
        println!("  Fully validated: {}", fully_validated.to_string().green());
        println!(
            "  Partially validated: {}",
            partially_validated.to_string().yellow()
        );
        println!("  Not validated: {}", unvalidated.to_string().red());
    }
}
