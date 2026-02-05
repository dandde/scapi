//! Scrape (fetch + select) handler.

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;

/// Scrape request payload.
#[derive(Debug, Deserialize)]
pub struct ScrapeRequest {
    /// URL to scrape
    pub url: String,
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
    /// Optional timeout in milliseconds
    pub timeout_ms: Option<u64>,
    /// Optional user agent
    pub user_agent: Option<String>,
}

/// Scrape response payload.
#[derive(Debug, Serialize)]
pub struct ScrapeResponse {
    /// Scraped data
    pub data: Vec<ScrapeMatch>,
    /// Number of matches
    pub count: usize,
    /// Detailed metrics
    pub metrics: ScrapeMetrics,
    /// Request metadata
    pub metadata: ResponseMetadata,
}

/// A scraped match.
#[derive(Debug, Serialize)]
pub struct ScrapeMatch {
    /// Text content
    pub text: String,
    /// HTML content
    pub html: String,
    /// Element attributes
    pub attributes: std::collections::HashMap<String, String>,
}

/// Scrape operation metrics.
#[derive(Debug, Serialize)]
pub struct ScrapeMetrics {
    /// Time spent fetching in milliseconds
    pub fetch_ms: u128,
    /// Time spent parsing in milliseconds
    pub parse_ms: u128,
    /// Time spent selecting in milliseconds
    pub select_ms: u128,
    /// Total time in milliseconds
    pub total_ms: u128,
    /// Content size in bytes
    pub content_size_bytes: usize,
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

/// Scrape URL and extract elements using selector.
pub async fn scrape_handler(
    State(_state): State<Arc<AppState>>,
    Json(_request): Json<ScrapeRequest>,
) -> impl IntoResponse {
    // TODO: Implement scrape logic
    (
        StatusCode::NOT_IMPLEMENTED,
        "Scrape handler not yet implemented".to_string(),
    )
}
