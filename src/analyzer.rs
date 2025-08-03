//! Core analysis functionality for detecting zero address validation patterns

use std::fs;
use std::path::Path;

use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;

use crate::types::{AnalysisResult, ConstructorAnalyzerError, FunctionType, ValidationType};

/// Main analyzer struct
pub struct ConstructorAnalyzer {
    constructor_regex: Regex,
    initialize_regex: Regex,
    function_regex: Regex,
    address_regex: Regex,
    equality_regex: Regex,
    require_regex: Regex,
}

impl ConstructorAnalyzer {
    /// Creates a new ConstructorAnalyzer instance
    pub fn new() -> Result<Self, ConstructorAnalyzerError> {
        let constructor_regex = RegexBuilder::new(r"constructor\s*\((.*?)\)\s*\{(.*?)\}")
            .multi_line(true)
            .dot_matches_new_line(true)
            .build()?;

        let initialize_regex =
            RegexBuilder::new(r"function\s+initialize\s*\((.*?)\)\s*[^{]*\{(.*?)\}")
                .multi_line(true)
                .dot_matches_new_line(true)
                .build()?;

        // Regex to match all functions (excluding constructor and initialize)
        let function_regex = RegexBuilder::new(r"function\s+(\w+)\s*\((.*?)\)\s*[^{]*\{(.*?)\}")
            .multi_line(true)
            .dot_matches_new_line(true)
            .build()?;

        let address_regex =
            Regex::new(r"(address(?:\[\])?(?:\s+memory|\s+storage|\s+calldata)?)\s+(\w+)")?;
        let equality_regex = Regex::new(r"(\w+)\s*(?:==|!=)\s*address\(0\)")?;
        let require_regex = Regex::new(r"(?:require)\s*\(\s*([^,)]+)\s*(?:==|!=)\s*address\(0\)")?;

        Ok(Self {
            constructor_regex,
            initialize_regex,
            function_regex,
            address_regex,
            equality_regex,
            require_regex,
        })
    }

    /// Analyzes constructors and initialize functions in the given path
    pub fn analyze_path(
        &self,
        path: &str,
        all_functions: bool,
    ) -> Result<Vec<AnalysisResult>, ConstructorAnalyzerError> {
        let path = Path::new(path);

        if !path.exists() {
            return Err(ConstructorAnalyzerError::NotFound(
                path.to_string_lossy().to_string(),
            ));
        }

        let mut results = Vec::new();

        if path.is_dir() {
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "sol"))
            {
                let file_results = self.analyze_file(entry.path(), all_functions)?;
                results.extend(file_results);
            }
        } else {
            let file_results = self.analyze_file(path, all_functions)?;
            results.extend(file_results);
        }

        Ok(results)
    }

    /// Analyzes a single Solidity file
    pub fn analyze_file(
        &self,
        file_path: &Path,
        all_functions: bool,
    ) -> Result<Vec<AnalysisResult>, ConstructorAnalyzerError> {
        let contents = fs::read_to_string(file_path)?;
        let file_name = file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let mut results = Vec::new();

        // Analyze constructors
        for captures in self.constructor_regex.captures_iter(&contents) {
            let args = captures.get(1).map_or("", |m| m.as_str()).trim();
            let body = captures.get(2).map_or("", |m| m.as_str()).trim();

            // Construct the full function definition
            let full_function = format!("constructor({}) {{\n{}\n}}", args, body);

            let result = self.analyze_function(
                FunctionType::Constructor,
                file_name.clone(),
                args,
                &full_function,
            );
            results.push(result);
        }

        // Analyze initialize functions
        for captures in self.initialize_regex.captures_iter(&contents) {
            let args = captures.get(1).map_or("", |m| m.as_str()).trim();
            let body = captures.get(2).map_or("", |m| m.as_str()).trim();

            // Construct the full function definition
            let full_function = format!("function initialize({}) {{\n{}\n}}", args, body);

            let result = self.analyze_function(
                FunctionType::Initialize,
                file_name.clone(),
                args,
                &full_function,
            );
            results.push(result);
        }

        // Analyze all other functions (only if all_functions flag is enabled)
        if all_functions {
            for captures in self.function_regex.captures_iter(&contents) {
                let function_name = captures.get(1).map_or("", |m| m.as_str()).trim();
                let args = captures.get(2).map_or("", |m| m.as_str()).trim();
                let body = captures.get(3).map_or("", |m| m.as_str()).trim();

                // Skip initialize function as we've already analyzed it above
                if function_name == "initialize" {
                    continue;
                }

                // Only analyze functions that have address parameters
                let address_arguments = self.extract_address_arguments(args);
                if !address_arguments.is_empty() {
                    // Construct the full function definition
                    let full_function =
                        format!("function {}({}) {{\n{}\n}}", function_name, args, body);

                    let result = self.analyze_function(
                        FunctionType::Regular(function_name.to_string()),
                        file_name.clone(),
                        args,
                        &full_function,
                    );
                    results.push(result);
                }
            }
        }

        Ok(results)
    }

    /// Analyzes a single function for zero address validation
    fn analyze_function(
        &self,
        function_type: FunctionType,
        file_name: String,
        arguments: &str,
        full_function_code: &str,
    ) -> AnalysisResult {
        let address_arguments = self.extract_address_arguments(arguments);

        // Extract just the body for validation pattern analysis
        // The full_function_code contains the complete function, but we need just the body for pattern matching
        let function_body = if let Some(start) = full_function_code.find('{') {
            if let Some(end) = full_function_code.rfind('}') {
                &full_function_code[start + 1..end].trim()
            } else {
                full_function_code
            }
        } else {
            full_function_code
        };

        let equality_vars = self.extract_equality_checked_variables(function_body);
        let require_vars = self.extract_require_checked_variables(function_body);

        let mut validated_variables = equality_vars.clone();
        for var in require_vars.iter() {
            if !validated_variables.contains(var) {
                validated_variables.push(var.clone());
            }
        }

        let missing_validations: Vec<String> = address_arguments
            .iter()
            .filter(|arg| !validated_variables.contains(&arg.1))
            .map(|arg| arg.1.clone())
            .collect();

        let mut validation_types = Vec::new();
        if !equality_vars.is_empty() {
            validation_types.push(ValidationType::EqualityCheck);
        }
        if !require_vars.is_empty() {
            validation_types.push(ValidationType::RequireStatement);
        }

        AnalysisResult {
            function_type,
            file_name,
            arguments: arguments.to_string(),
            code: full_function_code.to_string(),
            address_arguments,
            validated_variables,
            missing_validations,
            validation_types,
        }
    }

    /// Extracts address arguments from function parameters
    fn extract_address_arguments(&self, args: &str) -> Vec<(String, String)> {
        self.address_regex
            .captures_iter(args)
            .filter_map(|cap| {
                if let (Some(type_match), Some(name_match)) = (cap.get(1), cap.get(2)) {
                    Some((
                        type_match.as_str().to_string(),
                        name_match.as_str().to_string(),
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Extracts variables checked with equality operators
    fn extract_equality_checked_variables(&self, code: &str) -> Vec<String> {
        let mut variables = Vec::new();

        for captures in self.equality_regex.captures_iter(code) {
            if let Some(var_match) = captures.get(1) {
                let var_name = var_match.as_str().to_string();
                if !variables.contains(&var_name) {
                    variables.push(var_name);
                }
            }
        }

        variables
    }

    /// Extracts variables checked in require statements
    fn extract_require_checked_variables(&self, code: &str) -> Vec<String> {
        let mut variables = Vec::new();

        for captures in self.require_regex.captures_iter(code) {
            if let Some(var_match) = captures.get(1) {
                let var_name = var_match.as_str().trim().to_string();
                if !variables.contains(&var_name) {
                    variables.push(var_name);
                }
            }
        }

        variables
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_address_arguments() {
        let analyzer = ConstructorAnalyzer::new().unwrap();

        let args = "address _owner, uint256 _amount, address _token";
        let result = analyzer.extract_address_arguments(args);

        assert_eq!(
            result,
            vec![
                ("address".to_string(), "_owner".to_string()),
                ("address".to_string(), "_token".to_string())
            ]
        );
    }

    #[test]
    fn test_extract_address_arguments_with_arrays() {
        let analyzer = ConstructorAnalyzer::new().unwrap();

        let args = "address _owner, address[] memory _addresses, uint256 _amount, address[] calldata _tokens";
        let result = analyzer.extract_address_arguments(args);

        assert_eq!(
            result,
            vec![
                ("address".to_string(), "_owner".to_string()),
                ("address[] memory".to_string(), "_addresses".to_string()),
                ("address[] calldata".to_string(), "_tokens".to_string())
            ]
        );
    }

    #[test]
    fn test_extract_address_arguments_with_storage_keywords() {
        let analyzer = ConstructorAnalyzer::new().unwrap();

        let args = "address storage _owner, address[] storage _addresses, address calldata _token";
        let result = analyzer.extract_address_arguments(args);

        assert_eq!(
            result,
            vec![
                ("address storage".to_string(), "_owner".to_string()),
                ("address[] storage".to_string(), "_addresses".to_string()),
                ("address calldata".to_string(), "_token".to_string())
            ]
        );
    }

    #[test]
    fn test_extract_equality_checked_variables() {
        let analyzer = ConstructorAnalyzer::new().unwrap();

        let code = "if (_owner == address(0)) revert(); require(_token != address(0));";
        let result = analyzer.extract_equality_checked_variables(code);

        assert_eq!(result, vec!["_owner", "_token"]);
    }

    #[test]
    fn test_extract_require_checked_variables() {
        let analyzer = ConstructorAnalyzer::new().unwrap();

        let code =
            "require(_owner != address(0), \"Invalid owner\"); require(_token == address(0));";
        let result = analyzer.extract_require_checked_variables(code);

        assert_eq!(result, vec!["_owner", "_token"]);
    }

    #[test]
    fn test_analyze_function_with_missing_validation() {
        let analyzer = ConstructorAnalyzer::new().unwrap();

        let result = analyzer.analyze_function(
            FunctionType::Constructor,
            "Test.sol".to_string(),
            "address _owner, address _token",
            "require(_owner != address(0), \"Invalid owner\");",
        );

        assert_eq!(
            result.address_arguments,
            vec![
                ("address".to_string(), "_owner".to_string()),
                ("address".to_string(), "_token".to_string())
            ]
        );
        assert_eq!(result.validated_variables, vec!["_owner"]);
        assert_eq!(result.missing_validations, vec!["_token"]);
        assert!(
            result
                .validation_types
                .contains(&ValidationType::RequireStatement)
        );
    }

    #[test]
    fn test_analyze_function_fully_validated() {
        let analyzer = ConstructorAnalyzer::new().unwrap();

        let result = analyzer.analyze_function(
            FunctionType::Initialize,
            "Test.sol".to_string(),
            "address _owner, address _token",
            "require(_owner != address(0)); if (_token == address(0)) revert();",
        );

        assert_eq!(
            result.address_arguments,
            vec![
                ("address".to_string(), "_owner".to_string()),
                ("address".to_string(), "_token".to_string())
            ]
        );
        assert_eq!(result.validated_variables, vec!["_owner", "_token"]);
        assert!(result.missing_validations.is_empty());
        assert!(
            result
                .validation_types
                .contains(&ValidationType::RequireStatement)
        );
        assert!(
            result
                .validation_types
                .contains(&ValidationType::EqualityCheck)
        );
    }

    #[test]
    fn test_analyze_regular_function() {
        let analyzer = ConstructorAnalyzer::new().unwrap();

        let result = analyzer.analyze_function(
            FunctionType::Regular("transfer".to_string()),
            "Test.sol".to_string(),
            "address _to, uint256 _amount",
            "balances[_to] += _amount;",
        );

        assert_eq!(
            result.address_arguments,
            vec![("address".to_string(), "_to".to_string())]
        );
        assert!(result.validated_variables.is_empty());
        assert_eq!(result.missing_validations, vec!["_to"]);
        assert!(result.validation_types.is_empty());
    }

    #[test]
    fn test_analyze_regular_function_with_validation() {
        let analyzer = ConstructorAnalyzer::new().unwrap();

        let result = analyzer.analyze_function(
            FunctionType::Regular("setOwner".to_string()),
            "Test.sol".to_string(),
            "address _newOwner",
            "require(_newOwner != address(0), \"Invalid owner\"); owner = _newOwner;",
        );

        assert_eq!(
            result.address_arguments,
            vec![("address".to_string(), "_newOwner".to_string())]
        );
        assert_eq!(result.validated_variables, vec!["_newOwner"]);
        assert!(result.missing_validations.is_empty());
        assert!(
            result
                .validation_types
                .contains(&ValidationType::RequireStatement)
        );
    }
}
