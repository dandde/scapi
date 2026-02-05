//! Configuration for extract operations.

use serde::{Deserialize, Serialize};

/// Configuration for extract operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractConfig {
    /// Trim whitespace from extracted values
    pub trim_whitespace: bool,
    /// Decode HTML entities
    pub decode_html_entities: bool,
    /// Maximum number of fields to extract
    pub max_fields: usize,
    /// Validate extracted data types
    pub validate_types: bool,
    /// Default value for missing fields
    pub default_value: Option<String>,
    /// Strict mode (fail on first error)
    pub strict_mode: bool,
}

impl Default for ExtractConfig {
    fn default() -> Self {
        Self {
            trim_whitespace: true,
            decode_html_entities: true,
            max_fields: 100,
            validate_types: true,
            default_value: None,
            strict_mode: false,
        }
    }
}