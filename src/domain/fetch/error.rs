//! Error types for fetch operations.

use thiserror::Error;

/// Errors that can occur during fetch operations.
#[derive(Debug, Error)]
pub enum FetchError {
    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Network error (connection failed, DNS resolution, etc.)
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Request timeout
    #[error("Request timeout: {0}")]
    Timeout(String),

    /// Too many redirects
    #[error("Too many redirects: {0}")]
    TooManyRedirects(String),

    /// Server error (4xx or 5xx response)
    #[error("Server error: {0}")]
    ServerError(String),

    /// Content too large
    #[error("Content too large: {0}")]
    ContentTooLarge(String),

    /// Unsupported protocol
    #[error("Unsupported protocol: {0}")]
    UnsupportedProtocol(String),

    /// SSL/TLS error
    #[error("SSL/TLS error: {0}")]
    TlsError(String),

    /// Not implemented (temporary for development)
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}