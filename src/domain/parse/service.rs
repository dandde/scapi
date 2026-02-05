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
    /// The actual VDOM (needed for selection)
    pub vdom: std::sync::Arc<crate::infra::parser::VDom>,
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
    cache: std::sync::Arc<
        std::sync::Mutex<lru::LruCache<u64, std::sync::Arc<crate::infra::parser::VDom>>>,
    >,
}

impl DefaultParseService {
    /// Create a new parse service.
    pub fn new() -> Self {
        Self {
            parser: crate::infra::parser::html::HtmlParser::new(),
            cache: std::sync::Arc::new(std::sync::Mutex::new(lru::LruCache::new(
                std::num::NonZeroUsize::new(100).unwrap(),
            ))),
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
        let cache = self.cache.clone();
        let html_str = html.to_string();

        async move {
            // Start timing the operation
            let _timer = Timer::start("parse");

            // Calculate hash
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            html_str.hash(&mut hasher);
            let hash = hasher.finish();

            // Check cache
            {
                let mut cache_guard = cache.lock().unwrap();
                if let Some(vdom) = cache_guard.get(&hash) {
                    let vdom = vdom.clone();

                    // Re-calculate metrics from VDom
                    let total_elements = vdom.nodes.len();
                    let mut unique_tags = std::collections::HashSet::new();
                    for node in &vdom.nodes {
                        if !node.tag.is_empty() {
                            unique_tags.insert(node.tag.clone());
                        }
                    }
                    let root_children = vdom
                        .nodes
                        .get(vdom.root)
                        .map(|n| n.children.len())
                        .unwrap_or(0);

                    let structure = DomStructure {
                        root_tag: "document".to_string(),
                        child_count: root_children,
                        well_formed: true,
                        total_elements,
                        max_depth: 0,
                        unique_tags: unique_tags.into_iter().collect(),
                    };

                    return Ok(ParseResult {
                        total_elements,
                        max_depth: 0,
                        structure,
                        vdom,
                    });
                }
            }

            // Parse HTML
            let vdom = parser.parse(&html_str)?;
            let vdom_arc = std::sync::Arc::new(vdom);

            // Update cache
            {
                let mut cache_guard = cache.lock().unwrap();
                cache_guard.put(hash, vdom_arc.clone());
            }

            // Calculate metrics
            let total_elements = vdom_arc.nodes.len();
            let mut unique_tags = std::collections::HashSet::new();
            for node in &vdom_arc.nodes {
                if !node.tag.is_empty() {
                    unique_tags.insert(node.tag.clone());
                }
            }

            let root_children = if let Some(root) = vdom_arc.nodes.get(vdom_arc.root) {
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
                vdom: vdom_arc,
            })
        }
    }
}
