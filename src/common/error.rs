//! Common error types for SCAPI.

use thiserror::Error;

/// Common errors that can occur across multiple modules.
#[derive(Debug, Error)]
pub enum CommonError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Invalid input error
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Internal server error
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl CommonError {
    /// Create a new configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::ConfigError(msg.into())
    }

    /// Create a new invalid input error
    pub fn invalid_input(msg: impl Into<String>) -> Self {
        Self::InvalidInput(msg.into())
    }

    /// Create a new serialization error
    pub fn serialization(msg: impl Into<String>) -> Self {
        Self::SerializationError(msg.into())
    }

    /// Create a new internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::InternalError(msg.into())
    }
}

impl axum::response::IntoResponse for CommonError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::ConfigError(msg) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg),
            Self::InvalidInput(msg) => (axum::http::StatusCode::BAD_REQUEST, msg),
            Self::IoError(err) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            ),
            Self::SerializationError(msg) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg),
            Self::InternalError(msg) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = axum::Json(serde_json::json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}
