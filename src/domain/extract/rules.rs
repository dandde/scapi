//! Extraction rule definitions.

use serde::{Deserialize, Serialize};

/// Extraction rule type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtractionRule {
    /// Simple field extraction
    Simple(SimpleRule),
    /// Complex extraction with transformation
    Complex(ComplexRule),
    /// Computed field
    Computed(ComputedRule),
}

/// Simple extraction rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleRule {
    /// Field name
    pub field: String,
    /// Selector (CSS or XPath)
    pub selector: String,
    /// Selector type
    pub selector_type: SelectorType,
    /// Data type
    pub data_type: DataType,
    /// Whether the field is required
    pub required: bool,
    /// Default value if not found
    pub default: Option<String>,
}

/// Complex extraction rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexRule {
    /// Field name
    pub field: String,
    /// Multiple selectors (tried in order)
    pub selectors: Vec<String>,
    /// Data type
    pub data_type: DataType,
    /// Transformation function name
    pub transform: Option<String>,
    /// Validation rules
    pub validation: Vec<ValidationRule>,
}

/// Computed field rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputedRule {
    /// Field name
    pub field: String,
    /// Expression or function
    pub expression: String,
    /// Dependencies on other fields
    pub dependencies: Vec<String>,
    /// Data type
    pub data_type: DataType,
}

/// Selector type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelectorType {
    /// CSS selector
    Css,
    /// XPath selector
    XPath,
}

/// Data type for extracted values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    /// Text
    Text,
    /// Number (integer or float)
    Number,
    /// Boolean
    Boolean,
    /// Date/time
    DateTime,
    /// URL
    Url,
    /// Email
    Email,
}

/// Validation rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Validation type
    pub validation_type: ValidationType,
    /// Validation parameters
    pub params: std::collections::HashMap<String, String>,
}

/// Validation type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationType {
    /// Required field
    Required,
    /// Minimum length
    MinLength,
    /// Maximum length
    MaxLength,
    /// Pattern match (regex)
    Pattern,
    /// Minimum value
    MinValue,
    /// Maximum value
    MaxValue,
    /// Custom validation
    Custom,
}