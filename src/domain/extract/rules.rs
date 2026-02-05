//! Extraction rule definitions.

use serde::{Deserialize, Serialize};

/// Extraction rule definition.
///
/// A rule defines how to extract data from a node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionRule {
    /// Field name in the output JSON.
    pub field: String,

    /// Selector to find the element(s).
    /// If None, usage depends on context (e.g. current node).
    pub selector: Option<String>,

    /// Selector type (CSS vs XPath). Default: CSS.
    #[serde(default)]
    pub selector_type: SelectorType,

    /// Attribute to extract. If None, extracts text.
    pub attribute: Option<String>,

    /// Data type to cast to / treat as.
    #[serde(default)]
    pub data_type: DataType,

    /// If true, finds all matching elements and returns an Array.
    /// If false, finds the first match.
    #[serde(default)]
    pub multiple: bool,

    /// Nested rules. Used when data_type is Object, or when multiple is true and we want an array of objects.
    #[serde(default)]
    pub children: Vec<ExtractionRule>,

    /// Optional transformation steps (e.g. trim, regex).
    #[serde(default)]
    pub transform: Vec<TransformType>,
}

/// Selector type.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SelectorType {
    /// CSS selector
    #[default]
    Css,
    /// XPath selector
    XPath,
}

/// Data type for extracted values.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum DataType {
    /// Text (string)
    #[default]
    Text,
    /// Number (integer or float)
    Number,
    /// Boolean
    Boolean,
    /// Date/time string
    DateTime,
    /// URL string
    Url,
    /// Email string
    Email,
    /// Nested Object
    Object,
    /// Array of values (usually derived from `multiple`, but explicit type exists)
    Array,
}

/// Transformation types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformType {
    /// Trim whitespace
    Trim,
    /// Convert to lowercase
    Lowercase,
    /// Convert to uppercase
    Uppercase,
    /// Regex replacement (pattern, replacement)
    RegexReplace(String, String),
}
