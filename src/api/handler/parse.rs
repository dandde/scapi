//! Parse HTML content handler.

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;
use crate::domain::parse::service::ParseService;

/// Parse request payload.
#[derive(Debug, Deserialize)]
pub struct ParseRequest {
    /// HTML content to parse
    pub html: String,
    /// Optional: detect encoding
    pub detect_encoding: Option<bool>,
    /// Optional: handle malformed HTML
    pub handle_malformed: Option<bool>,
}

/// Parse response payload.
#[derive(Debug, Serialize)]
pub struct ParseResponse {
    /// Total number of elements
    pub total_elements: usize,
    /// Maximum depth of the DOM tree
    pub max_depth: usize,
    /// DOM structure information
    pub structure: DomStructure,
    /// Request metadata
    pub metadata: ResponseMetadata,
}

/// DOM structure information.
#[derive(Debug, Serialize)]
pub struct DomStructure {
    /// Root element tag
    pub root_tag: String,
    /// Number of direct children
    pub child_count: usize,
    /// Whether the HTML is well-formed
    pub well_formed: bool,
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

pub async fn parse_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ParseRequest>,
) -> impl IntoResponse {
    let config = crate::domain::parse::config::ParseConfig {
        ..Default::default()
    };

    match state.parse_service.parse(&request.html, &config).await {
        Ok(result) => {
            let response = ParseResponse {
                total_elements: result.total_elements,
                max_depth: result.max_depth,
                structure: DomStructure {
                    root_tag: result.structure.root_tag,
                    child_count: result.structure.child_count,
                    well_formed: result.structure.well_formed,
                },
                metadata: ResponseMetadata {
                    request_id: "TODO".to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    duration_ms: 0,
                },
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Parse failed: {}", e),
        )
            .into_response(),
    }
}
