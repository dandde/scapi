//! Error types for extract operations.

use thiserror::Error;

/// Errors that can occur during extract operations.
#[derive(Debug, Error)]
pub enum ExtractError {
    /// Invalid extraction rule
    #[error("Invalid extraction rule: {0}")]
    InvalidRule(String),

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Parsing error
    #[error("Parsing error: {0}")]
    ParsingError(String),

    /// Selection error
    #[error("Selection error: {0}")]
    SelectionError(String),

    /// Type conversion error
    #[error("Type conversion error: {0}")]
    ConversionError(String),

    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingRequiredField(String),

    /// Selector execution error
    #[error("Selector execution error: {0}")]
    SelectorError(String),

    /// Not implemented (temporary for development)
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}
