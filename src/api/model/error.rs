//! API error types and HTTP response conversions.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Serialize;
use thiserror::Error;

/// API error response format.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Error details
    pub error: ErrorDetails,
    /// Request ID for tracking
    pub request_id: String,
    /// Timestamp of the error
    pub timestamp: String,
}

/// Error details structure.
#[derive(Debug, Serialize)]
pub struct ErrorDetails {
    /// Error code
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Additional error details (optional)
    pub details: Option<serde_json::Value>,
}

/// Main API error type.
#[derive(Debug, Error)]
pub enum ApiError {
    /// Validation errors (400)
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Bad request (400)
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Invalid URL (400)
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Invalid selector (400)
    #[error("Invalid selector: {0}")]
    InvalidSelector(String),

    /// Missing required field (400)
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Unauthorized (401)
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// Rate limited (429)
    #[error("Rate limited: {0}")]
    RateLimited(String),

    /// Not found (404)
    #[error("Not found: {0}")]
    NotFound(String),

    /// Internal server error (500)
    #[error("Internal server error: {0}")]
    InternalError(String),

    /// Service unavailable (503)
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    /// Timeout (504)
    #[error("Timeout: {0}")]
    Timeout(String),
}

impl ApiError {
    /// Get the HTTP status code for this error.
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::InvalidUrl(_) => StatusCode::BAD_REQUEST,
            ApiError::InvalidSelector(_) => StatusCode::BAD_REQUEST,
            ApiError::MissingField(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::RateLimited(_) => StatusCode::TOO_MANY_REQUESTS,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            ApiError::Timeout(_) => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    /// Get the error code string.
    pub fn error_code(&self) -> &'static str {
        match self {
            ApiError::ValidationError(_) => "VALIDATION_ERROR",
            ApiError::BadRequest(_) => "BAD_REQUEST",
            ApiError::InvalidUrl(_) => "INVALID_URL",
            ApiError::InvalidSelector(_) => "INVALID_SELECTOR",
            ApiError::MissingField(_) => "MISSING_FIELD",
            ApiError::Unauthorized(_) => "UNAUTHORIZED",
            ApiError::RateLimited(_) => "RATE_LIMITED",
            ApiError::NotFound(_) => "NOT_FOUND",
            ApiError::InternalError(_) => "INTERNAL_ERROR",
            ApiError::ServiceUnavailable(_) => "SERVICE_UNAVAILABLE",
            ApiError::Timeout(_) => "TIMEOUT",
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let error_response = ErrorResponse {
            error: ErrorDetails {
                code: self.error_code().to_string(),
                message: self.to_string(),
                details: None,
            },
            request_id: "".to_string(), // Will be populated by middleware
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        (status, Json(error_response)).into_response()
    }
}

/// Convenience type alias for API results.
pub type ApiResult<T> = Result<T, ApiError>;