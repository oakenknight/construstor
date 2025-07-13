//! Types and data structures used throughout the application

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Custom error type for the application
#[derive(Debug)]
pub enum ConstructorAnalyzerError {
    /// IO error occurred while reading files
    IoError(std::io::Error),
    /// Invalid regex pattern
    RegexError(regex::Error),
    /// File or directory not found
    NotFound(String),
    /// Invalid file format
    InvalidFormat(String),
}

impl fmt::Display for ConstructorAnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstructorAnalyzerError::IoError(err) => write!(f, "IO error: {err}"),
            ConstructorAnalyzerError::RegexError(err) => write!(f, "Regex error: {err}"),
            ConstructorAnalyzerError::NotFound(path) => write!(f, "Path not found: {path}"),
            ConstructorAnalyzerError::InvalidFormat(msg) => write!(f, "Invalid format: {msg}"),
        }
    }
}

impl Error for ConstructorAnalyzerError {}

impl From<std::io::Error> for ConstructorAnalyzerError {
    fn from(err: std::io::Error) -> Self {
        ConstructorAnalyzerError::IoError(err)
    }
}

impl From<regex::Error> for ConstructorAnalyzerError {
    fn from(err: regex::Error) -> Self {
        ConstructorAnalyzerError::RegexError(err)
    }
}

/// Represents the result of analyzing a function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Function type (Constructor or Initialize)
    pub function_type: FunctionType,
    /// File name where the function was found
    pub file_name: String,
    /// Function arguments
    pub arguments: String,
    /// Function body code
    pub code: String,
    /// Address arguments found in the function signature
    pub address_arguments: Vec<String>,
    /// Variables that have zero address validation
    pub validated_variables: Vec<String>,
    /// Variables that are missing zero address validation
    pub missing_validations: Vec<String>,
    /// Types of validation found
    pub validation_types: Vec<ValidationType>,
}

/// Type of function being analyzed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FunctionType {
    Constructor,
    Initialize,
}

/// Type of zero address validation found
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationType {
    EqualityCheck,
    RequireStatement,
}
