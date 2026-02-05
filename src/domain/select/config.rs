//! Configuration for select operations.

use serde::{Deserialize, Serialize};

/// Selector type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SelectorType {
    /// CSS selector
    Css,
    /// XPath selector
    XPath,
}

impl Default for SelectorType {
    fn default() -> Self {
        Self::Css
    }
}

/// Configuration for select operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectConfig {
    /// Return only text content
    pub text_only: bool,
    /// Return only first match
    pub first_only: bool,
    /// Maximum number of results
    pub max_results: usize,
    /// Selector type
    pub selector_type: SelectorType,
    /// Include HTML attributes in results
    pub include_attributes: bool,
    /// Include HTML content in results
    pub include_html: bool,
}

impl Default for SelectConfig {
    fn default() -> Self {
        Self {
            text_only: false,
            first_only: false,
            max_results: 10000,
            selector_type: SelectorType::Css,
            include_attributes: true,
            include_html: false,
        }
    }
}