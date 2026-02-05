//! Virtual DOM abstraction.

use std::collections::HashMap;

/// Node ID in the VDOM.
pub type NodeId = usize;

/// Virtual DOM representation.
#[derive(Debug, Clone)]
pub struct VDom {
    /// All nodes in the DOM
    pub nodes: Vec<Node>,
    /// Root node ID
    pub root: NodeId,
}

/// DOM node.
#[derive(Debug, Clone)]
pub struct Node {
    /// Tag name
    pub tag: String,
    /// HTML attributes
    pub attributes: HashMap<String, String>,
    /// Text content
    pub text: Option<String>,
    /// Child node IDs
    pub children: Vec<NodeId>,
    /// Parent node ID
    pub parent: Option<NodeId>,
}

impl VDom {
    /// Create a new empty VDOM.
    pub fn new() -> Self {
        let root = Node {
            tag: "root".to_string(),
            attributes: HashMap::new(),
            text: None,
            children: Vec::new(),
            parent: None,
        };

        Self {
            nodes: vec![root],
            root: 0,
        }
    }

    /// Query the VDOM using a selector (placeholder).
    pub fn query(&self, _selector: &str) -> Vec<NodeId> {
        // TODO: Implement selector query
        vec![]
    }

    /// Get a node by ID.
    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// Add a node to the VDOM.
    pub fn add_node(&mut self, node: Node) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }

    /// Reconstruct HTML for a node, filtering out excluded IDs.
    pub fn reconstruct_html(
        &self,
        node_id: NodeId,
        exclude_ids: &std::collections::HashSet<NodeId>,
    ) -> String {
        if exclude_ids.contains(&node_id) {
            return String::new();
        }

        if let Some(node) = self.get_node(node_id) {
            if node.tag == "text" {
                return node.text.clone().unwrap_or_default();
            }
            if node.tag == "comment" {
                // Skip comments in reconstruction to be cleaner, or keep them?
                // Standard behavior usually keeps them unless filtered.
                // For now, let's include them as <!-- ... --> if we had content, but tl stores comments differently.
                // Our parser implementation marked tag="comment".
                return String::new(); // Simplified: ignore comments for now
            }
            if node.tag == "document" || node.tag == "root" {
                // For root/document, just render children
                return node
                    .children
                    .iter()
                    .map(|&child_id| self.reconstruct_html(child_id, exclude_ids))
                    .collect();
            }

            let mut html = String::new();
            html.push('<');
            html.push_str(&node.tag);

            // Sort attributes for deterministic output (benchmarking friendly)
            let mut attrs: Vec<_> = node.attributes.iter().collect();
            attrs.sort_by(|a, b| a.0.cmp(b.0));

            for (key, value) in attrs {
                html.push(' ');
                html.push_str(key);
                if !value.is_empty() {
                    html.push_str("=\"");
                    html.push_str(&html_escape::encode_double_quoted_attribute(value));
                    html.push('"');
                }
            }

            // Self-closing tags logic could act here, but VDOM usually represents full tree.
            // Let's assume standard opening/closing for now.
            html.push('>');

            for &child_id in &node.children {
                html.push_str(&self.reconstruct_html(child_id, exclude_ids));
            }

            // Void elements Check (e.g. meta, img, br, hr, input)
            // Ideally we should know void elements.
            // List: area, base, br, col, embed, hr, img, input, link, meta, param, source, track, wbr
            let void_tags = [
                "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta",
                "param", "source", "track", "wbr",
            ];
            if !void_tags.contains(&node.tag.as_str()) {
                html.push_str("</");
                html.push_str(&node.tag);
                html.push('>');
            }

            return html;
        }
        String::new()
    }
}

impl Default for VDom {
    fn default() -> Self {
        Self::new()
    }
}
