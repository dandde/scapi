//! Parse result models.

use serde::{Deserialize, Serialize};

/// DOM structure information.
#[derive(Debug, Serialize, Deserialize)]
pub struct DomStructure {
    /// Root element tag
    pub root_tag: String,
    /// Number of direct children
    pub child_count: usize,
    /// Whether the HTML is well-formed
    pub well_formed: bool,
    /// Total number of elements
    pub total_elements: usize,
    /// Maximum depth
    pub max_depth: usize,
    /// List of unique tags
    pub unique_tags: Vec<String>,
}

/// Element metadata.
#[derive(Debug, Serialize, Deserialize)]
pub struct Element {
    /// Tag name
    pub tag: String,
    /// Text content
    pub text: Option<String>,
    /// HTML attributes
    pub attributes: std::collections::HashMap<String, String>,
    /// Children elements
    pub children: Vec<Element>,
    /// Depth in the tree
    pub depth: usize,
}

/// Parse statistics.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParseStats {
    /// Time taken in milliseconds
    pub time_ms: u128,
    /// Memory usage in bytes
    pub memory_bytes: usize,
    /// Number of elements processed
    pub elements_processed: usize,
}