//! Configuration for parse operations.

use serde::{Deserialize, Serialize};

/// Configuration for parse operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseConfig {
    /// Detect encoding automatically
    pub detect_encoding: bool,
    /// Handle malformed HTML
    pub handle_malformed: bool,
    /// Maximum HTML size in bytes
    pub max_size_bytes: usize,
    /// Extract attributes during parsing
    pub extract_attributes: bool,
    /// Include hierarchy information
    pub include_hierarchy: bool,
}

impl Default for ParseConfig {
    fn default() -> Self {
        Self {
            detect_encoding: true,
            handle_malformed: true,
            max_size_bytes: 100 * 1024 * 1024, // 100MB
            extract_attributes: true,
            include_hierarchy: false,
        }
    }
}