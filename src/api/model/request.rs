//! Request types for SCAPI endpoints.

use serde::Deserialize;

/// Fetch request type.
#[derive(Debug, Deserialize)]
pub struct FetchRequest {
    /// URL to fetch
    pub url: String,
    /// Optional timeout in milliseconds
    pub timeout_ms: Option<u64>,
    /// Optional user agent
    pub user_agent: Option<String>,
}

/// Parse request type.
#[derive(Debug, Deserialize)]
pub struct ParseRequest {
    /// HTML content to parse
    pub html: String,
    /// Optional: detect encoding
    pub detect_encoding: Option<bool>,
    /// Optional: handle malformed HTML
    pub handle_malformed: Option<bool>,
}

/// Select request type.
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

/// Scrape request type.
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

/// Extract request type.
#[derive(Debug, Deserialize)]
pub struct ExtractRequest {
    /// HTML content or selected elements
    pub content: String,
    /// Extraction rules
    pub rules: Vec<ExtractionRule>,
    /// Trim whitespace
    pub trim_whitespace: Option<bool>,
    /// Decode HTML entities
    pub decode_html_entities: Option<bool>,
    /// Maximum number of fields
    pub max_fields: Option<usize>,
}

/// Extraction rule type.
#[derive(Debug, Deserialize)]
pub struct ExtractionRule {
    /// Field name
    pub field: String,
    /// Selector for the field
    pub selector: String,
    /// Data type (text, number, boolean, etc.)
    pub data_type: String,
    /// Whether the field is required
    pub required: bool,
}