//! Error types for parse operations.

use thiserror::Error;

/// Errors that can occur during parse operations.
#[derive(Debug, Error)]
pub enum ParseError {
    /// Invalid HTML
    #[error("Invalid HTML: {0}")]
    InvalidHtml(String),

    /// Encoding error
    #[error("Encoding error: {0}")]
    EncodingError(String),

    /// HTML size exceeded limit
    #[error("HTML size exceeded limit: {0}")]
    SizeExceeded(String),

    /// Parser error
    #[error("Parser error: {0}")]
    ParserError(String),

    /// Not implemented (temporary for development)
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("Parsing failed: {0}")]
    ParsingFailed(String),

    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}
