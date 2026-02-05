//! Health check handler.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Serialize;

/// Health check response.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// Service status
    pub status: String,
    /// Service version
    pub version: String,
    /// Service uptime in seconds (placeholder)
    pub uptime_seconds: u64,
}

/// Health check handler.
///
/// Returns service status and version information.
pub async fn health_handler() -> impl IntoResponse {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: 0, // Will be implemented when we have uptime tracking
    };

    (StatusCode::OK, Json(response))
}