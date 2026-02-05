//! Select elements from HTML handler.

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;

/// Select request payload.
#[derive(Debug, Deserialize)]
pub struct SelectRequest {
    /// HTML content or parsed structure
    pub html: String,
    /// CSS or XPath selector
    pub selector: String,
    /// Selector type (css or xpath)
    pub selector_type: Option<String>,
    /// Return only text content
    pub text_only: Option<bool>,
    /// Return only first match
    pub first_only: Option<bool>,
    /// Maximum number of results
    pub max_results: Option<usize>,
}

/// Select response payload.
#[derive(Debug, Serialize)]
pub struct SelectResponse {
    /// Matched elements
    pub matches: Vec<Match>,
    /// Total number of matches
    pub count: usize,
    /// Selector type used
    pub selector_type: String,
    /// Request metadata
    pub metadata: ResponseMetadata,
}

/// A matched element.
#[derive(Debug, Serialize)]
pub struct Match {
    /// Element tag name
    pub tag: String,
    /// Text content
    pub text: Option<String>,
    /// HTML attributes
    pub attributes: std::collections::HashMap<String, String>,
    /// HTML content
    pub html: String,
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

/// Select elements from HTML using CSS/XPath.
pub async fn select_handler(
    State(_state): State<Arc<AppState>>,
    Json(_request): Json<SelectRequest>,
) -> impl IntoResponse {
    // TODO: Implement select logic
    (
        StatusCode::NOT_IMPLEMENTED,
        "Select handler not yet implemented".to_string(),
    )
}
