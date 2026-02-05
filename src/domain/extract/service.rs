//! Extract service implementation.

use crate::common::metrics::Timer;
use crate::domain::parse::service::ParseService; // Import trait to use methods

use crate::infra::parser::vdom::Node; // Import Node

use super::config::ExtractConfig;
use super::error::ExtractError;
use super::rules::ExtractionRule;
use serde::{Deserialize, Serialize};

/// Extracted value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtractedValue {
    /// Text value
    Text(String),
    /// Numeric value
    Number(f64),
    /// Boolean value
    Boolean(bool),
    /// Array of values
    Array(Vec<ExtractedValue>),
    /// Object (map) of values
    Object(std::collections::HashMap<String, ExtractedValue>),
}

/// Extraction statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Result of an extract operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractResult {
    /// Extracted data
    pub data: Vec<ExtractedValue>,
    /// Validation errors
    pub validation_errors: Vec<String>,
    /// Extraction statistics
    pub stats: ExtractionStats,
}

/// Trait for extract services.
pub trait ExtractService: Send + Sync {
    /// Extract structured data from HTML using rules.
    fn extract(
        &self,
        html: &str,
        rules: &[ExtractionRule],
        config: &ExtractConfig,
    ) -> impl std::future::Future<Output = Result<ExtractResult, ExtractError>> + Send;
}

/// Default implementation of the extract service.
#[derive(Clone)]
pub struct DefaultExtractService {
    parse_service: std::sync::Arc<crate::domain::parse::service::DefaultParseService>,
}

impl DefaultExtractService {
    /// Create a new extract service.
    pub fn new(
        parse_service: std::sync::Arc<crate::domain::parse::service::DefaultParseService>,
    ) -> Self {
        Self { parse_service }
    }

    #[allow(dead_code)]

    fn extract_single(
        &self,
        node: &Node,
        rule: &ExtractionRule,
        vdom: &crate::infra::parser::VDom,
    ) -> ExtractedValue {
        // 1. Selector/Node resolution is done by caller usually, but if this is recursive children extraction:
        // The `node` here is the context node.

        let value_str = if let Some(attr_name) = &rule.attribute {
            node.attributes.get(attr_name).cloned().unwrap_or_default()
        } else {
            get_text_content(node, vdom)
        };

        // TODO: Apply transforms here

        // Convert type
        match rule.data_type {
            super::rules::DataType::Text => ExtractedValue::Text(value_str),
            super::rules::DataType::Number => {
                // simple parsing
                let num = value_str.trim().parse::<f64>().unwrap_or(0.0);
                ExtractedValue::Number(num)
            }
            super::rules::DataType::Boolean => {
                ExtractedValue::Boolean(value_str.trim().eq_ignore_ascii_case("true"))
            }
            // For Object/Array, we shouldn't really be here unless `multiple` was false but type is Object
            // If type is Object, we need to process children against THIS node
            super::rules::DataType::Object => {
                let map = std::collections::HashMap::new();
                for child_rule in &rule.children {
                    // Logic to find child element relative to current node
                    // But we need `SelectService` or manual logic to find descendants of *this* node.
                    // SelectService `select` takes `VDom` and returns IDs. Not scoped to a specific node currently?
                    // CssSelector logic supports context?
                    // CssSelector::select returns ALL matches in VDOM.
                    // We need scoped selection!
                    // Workaround: Use a selector that chains? No.
                    // We need `select_from_node(vdom, node_id, selector)`.
                    // For now, let's just return empty object to compile, fix logic next step.
                    let _ = child_rule;
                    let _ = vdom;
                }
                ExtractedValue::Object(map)
            }
            _ => ExtractedValue::Text(value_str),
        }
    }
}

impl ExtractService for DefaultExtractService {
    fn extract(
        &self,
        html: &str,
        rules: &[ExtractionRule],
        config: &ExtractConfig,
    ) -> impl std::future::Future<Output = Result<ExtractResult, ExtractError>> + Send {
        let parse_service = self.parse_service.clone();

        let html_str = html.to_string();
        let rules_vec = rules.to_vec();
        let _config_clone = config.clone();
        let _service = self.clone();

        async move {
            let timer = Timer::start("extract");

            // 1. Parse (using parse_service directly)
            let parse_config = crate::domain::parse::config::ParseConfig::default(); // TODO: map config
            let parse_result = parse_service
                .parse(&html_str, &parse_config)
                .await
                .map_err(|e| ExtractError::ParsingError(e.to_string()))?;

            let _vdom = parse_result.vdom;

            let data = Vec::new();
            let successful = 0;
            let failed = 0;

            // 2. Process root rules
            for rule in &rules_vec {
                // If selector is present, find nodes.
                if let Some(_selector) = &rule.selector {
                    // TODO: Selector implementation has been removed.
                    // This logic needs to be re-implemented or adapted if selection is still required for extraction.
                    // For now, we skip selector-based extraction to allow compilation.
                    // See implementation_plan.md for details.
                    tracing::warn!("Selector-based extraction is currently disabled.");
                }
            }

            Ok(ExtractResult {
                data,
                validation_errors: vec![],
                stats: ExtractionStats {
                    total_fields: rules_vec.len(),
                    successful,
                    failed,
                    time_ms: timer.finish_ms(),
                },
            })
        }
    }
}

/// Helper to get recursive text content.
#[allow(dead_code)]
fn get_text_content(node: &Node, vdom: &crate::infra::parser::VDom) -> String {
    if let Some(text) = &node.text {
        return text.clone();
    }

    let mut result = String::new();
    for &child_id in &node.children {
        if let Some(child) = vdom.nodes.get(child_id) {
            result.push_str(&get_text_content(child, vdom));
        }
    }
    result
}
