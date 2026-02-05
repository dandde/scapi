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

/// Fetch HTML content from a URL with streaming.
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

    match state
        .fetch_service
        .fetch_stream(&request.url, &config)
        .await
    {
        Ok(result) => {
            // Convert ResponseStream to Axum Body
            let body = axum::body::Body::from_stream(result.stream);

            let mut response = body.into_response();

            // Set headers for metadata
            response.headers_mut().insert(
                "X-Scapi-Status-Code",
                axum::http::HeaderValue::from(result.status_code),
            );

            if let Ok(url_header) = axum::http::HeaderValue::from_str(&result.final_url) {
                response
                    .headers_mut()
                    .insert("X-Scapi-Final-Url", url_header);
            }

            if let Some(len) = result.content_length {
                response.headers_mut().insert(
                    axum::http::header::CONTENT_LENGTH,
                    axum::http::HeaderValue::from(len),
                );
            }

            response.headers_mut().insert(
                "X-Scapi-Timestamp",
                axum::http::HeaderValue::from_str(&result.timestamp.to_rfc3339())
                    .unwrap_or_else(|_| axum::http::HeaderValue::from_static("unknown")),
            );

            response
        }
        Err(e) => {
            // TODO: specific error mapping
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Fetch failed: {}", e),
            )
                .into_response()
        }
    }
}
