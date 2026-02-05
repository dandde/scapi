//! Parse service implementation.

use crate::common::metrics::Timer;

use super::config::ParseConfig;
use super::error::ParseError;
use super::models::DomStructure;

/// Result of a parse operation.
#[derive(Debug)]
pub struct ParseResult {
    /// Total number of elements
    pub total_elements: usize,
    /// Maximum depth of the DOM tree
    pub max_depth: usize,
    /// DOM structure information
    pub structure: DomStructure,
}

/// Trait for parse services.
pub trait ParseService: Send + Sync {
    /// Parse HTML content into DOM structure.
    fn parse(
        &self,
        html: &str,
        config: &ParseConfig,
    ) -> impl std::future::Future<Output = Result<ParseResult, ParseError>> + Send;
}

/// Default implementation of the parse service.
pub struct DefaultParseService {
    parser: crate::infra::parser::html::HtmlParser,
}

impl DefaultParseService {
    /// Create a new parse service.
    pub fn new() -> Self {
        Self {
            parser: crate::infra::parser::html::HtmlParser::new(),
        }
    }
}

impl Default for DefaultParseService {
    fn default() -> Self {
        Self::new()
    }
}

impl ParseService for DefaultParseService {
    fn parse(
        &self,
        html: &str,
        _config: &ParseConfig,
    ) -> impl std::future::Future<Output = Result<ParseResult, ParseError>> + Send {
        let parser = self.parser.clone();
        let html = html.to_string();

        async move {
            // Start timing the operation
            let _timer = Timer::start("parse");

            // Parse HTML
            let vdom = parser.parse(&html)?;

            // Calculate metrics
            let total_elements = vdom.nodes.len();
            let mut unique_tags = std::collections::HashSet::new();
            for node in &vdom.nodes {
                if !node.tag.is_empty() {
                    unique_tags.insert(node.tag.clone());
                }
            }

            let root_children = if let Some(root) = vdom.nodes.get(vdom.root) {
                root.children.len()
            } else {
                0
            };

            let structure = DomStructure {
                root_tag: "document".to_string(),
                child_count: root_children,
                well_formed: true, // simplified assumption
                total_elements,
                max_depth: 0, // TODO: Calculate depth
                unique_tags: unique_tags.into_iter().collect(),
            };

            Ok(ParseResult {
                total_elements,
                max_depth: 0,
                structure,
            })
        }
    }
}
