//! Error types for select operations.

use thiserror::Error;

/// Errors that can occur during select operations.
#[derive(Debug, Error)]
pub enum SelectError {
    /// Invalid selector syntax
    #[error("Invalid selector: {0}")]
    InvalidSelector(String),

    /// No matches found
    #[error("No matches found: {0}")]
    NoMatches(String),

    /// Parser error
    #[error("Parser error: {0}")]
    ParseError(String),

    /// Execution error
    #[error("Execution error: {0}")]
    ExecutionError(String),

    /// Selector type not supported
    #[error("Selector type not supported: {0}")]
    SelectorTypeNotSupported(String),

    /// Not implemented (temporary for development)
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}