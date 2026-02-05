//! Response types for SCAPI endpoints.

use serde::Serialize;

/// Common response metadata.
#[derive(Debug, Serialize)]
pub struct ResponseMetadata {
    /// Request ID
    pub request_id: String,
    /// Timestamp in RFC3339 format
    pub timestamp: String,
    /// Duration in milliseconds
    pub duration_ms: u128,
}

/// Fetch response type.
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
    /// Response metadata
    pub metadata: ResponseMetadata,
}

/// Parse response type.
#[derive(Debug, Serialize)]
pub struct ParseResponse {
    /// Total number of elements
    pub total_elements: usize,
    /// Maximum depth of the DOM tree
    pub max_depth: usize,
    /// DOM structure information
    pub structure: DomStructure,
    /// Response metadata
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

/// Select response type.
#[derive(Debug, Serialize)]
pub struct SelectResponse {
    /// Matched elements
    pub matches: Vec<Match>,
    /// Total number of matches
    pub count: usize,
    /// Selector type used
    pub selector_type: String,
    /// Response metadata
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

/// Scrape response type.
#[derive(Debug, Serialize)]
pub struct ScrapeResponse {
    /// Scraped data
    pub data: Vec<ScrapeMatch>,
    /// Number of matches
    pub count: usize,
    /// Detailed metrics
    pub metrics: ScrapeMetrics,
    /// Response metadata
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

/// Extract response type.
#[derive(Debug, Serialize)]
pub struct ExtractResponse {
    /// Extracted data
    pub results: Vec<ExtractedData>,
    /// Validation errors
    pub validation_errors: Vec<String>,
    /// Extraction statistics
    pub stats: ExtractionStats,
    /// Response metadata
    pub metadata: ResponseMetadata,
}

/// Extracted data.
#[derive(Debug, Serialize)]
pub struct ExtractedData {
    /// Field values
    pub fields: std::collections::HashMap<String, serde_json::Value>,
}

/// Extraction statistics.
#[derive(Debug, Serialize)]
pub struct ExtractionStats {
    /// Total fields processed
    pub total_fields: usize,
    /// Successful extractions
    pub successful: usize,
    /// Failed extractions
    pub failed: usize,
    /// Time taken in milliseconds
    pub time_ms: u128,
}