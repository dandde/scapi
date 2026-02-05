//! Fetch HTML content handler.

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;
use crate::domain::fetch::service::FetchService;

/// Fetch request payload.
#[derive(Debug, Deserialize)]
pub struct FetchRequest {
    /// URL to fetch
    pub url: String,
    /// Optional timeout in milliseconds
    pub timeout_ms: Option<u64>,
    /// Optional user agent
    pub user_agent: Option<String>,
}

/// Fetch response payload.
#[derive(Debug, Serialize)]
pub struct FetchResponse {
    /// HTML content
    pub content: String,
    /// Content length in bytes
    pub length: usize,
    /// HTTP status code
    pub status_code: u16,
    /// Final URL after redirects
    pub final_url: String,
    /// Request metadata
    pub metadata: ResponseMetadata,
}

/// Response metadata (common to all responses).
#[derive(Debug, Serialize)]
pub struct ResponseMetadata {
    /// Request ID
    pub request_id: String,
    /// Timestamp
    pub timestamp: String,
    /// Duration in milliseconds
    pub duration_ms: u128,
}

/// Fetch HTML content from a URL.
pub async fn fetch_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<FetchRequest>,
) -> impl IntoResponse {
    let config = crate::domain::fetch::config::FetchConfig {
        timeout: std::time::Duration::from_millis(request.timeout_ms.unwrap_or(30000)),
        user_agent: request
            .user_agent
            .unwrap_or_else(|| "SCAPI/1.0".to_string()),
        ..Default::default()
    };

    match state.fetch_service.fetch(&request.url, &config).await {
        Ok(result) => {
            let response = FetchResponse {
                content: result.content,
                length: result.length,
                status_code: result.status_code,
                final_url: result.final_url,
                metadata: ResponseMetadata {
                    request_id: "TODO".to_string(), // Need to get from middleware/request context
                    timestamp: result.timestamp.to_rfc3339(),
                    duration_ms: 0, // TODO: Measure handler duration
                },
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            // TODO: Better error mapping
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Fetch failed: {}", e),
            )
                .into_response()
        }
    }
}
