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
}

impl Default for VDom {
    fn default() -> Self {
        Self::new()
    }
}