//! HTML parser wrapper.

use crate::domain::parse::error::ParseError;
use crate::infra::parser::vdom::{Node as VDomNode, VDom};
use std::collections::HashMap;
use tl;

/// HTML parser.
#[derive(Debug, Clone)]
pub struct HtmlParser;

impl HtmlParser {
    /// Create a new HTML parser.
    pub fn new() -> Self {
        Self
    }

    /// Parse HTML into a VDOM.
    pub fn parse(&self, html: &str) -> Result<VDom, ParseError> {
        let dom = tl::parse(html, tl::ParserOptions::default())
            .map_err(|e| ParseError::ParsingFailed(format!("TL parse error: {:?}", e)))?;

        let mut vdom = VDom::new();
        vdom.nodes.clear();

        // 0 is our document root
        let root_node = VDomNode {
            tag: "document".to_string(),
            attributes: HashMap::new(),
            text: None,
            children: Vec::new(),
            parent: None,
        };
        let root_id = vdom.add_node(root_node);
        vdom.root = root_id;

        // Pass 1: Create all nodes in VDom
        // tl sets up an arena of nodes. We can iterate them.
        let tl_nodes = dom.nodes();
        let offset = 1; // Our nodes will start at ID 1 (0 is root)

        for node in tl_nodes {
            let mut vdom_node = VDomNode {
                tag: String::new(),
                attributes: HashMap::new(),
                text: None,
                children: Vec::new(),
                parent: None, // Will fill in pass 2
            };

            match node {
                tl::Node::Tag(tag) => {
                    vdom_node.tag = tag.name().as_utf8_str().to_string();
                    for (k, v) in tag.attributes().iter() {
                        if let Some(val) = v {
                            vdom_node.attributes.insert(k.to_string(), val.to_string());
                        } else {
                            vdom_node.attributes.insert(k.to_string(), "".to_string());
                        }
                    }
                }
                tl::Node::Raw(bytes) => {
                    vdom_node.tag = "text".to_string();
                    vdom_node.text = Some(bytes.as_utf8_str().to_string());
                }
                tl::Node::Comment(_) => {
                    vdom_node.tag = "comment".to_string();
                }
            }
            vdom.add_node(vdom_node);
        }

        // Pass 2: Link children and parents
        // Also track which nodes have parents to find orphans (roots).
        let mut has_parent = vec![false; tl_nodes.len()];

        for (i, node) in tl_nodes.iter().enumerate() {
            let my_id = i + offset;

            if let Some(tag) = node.as_tag() {
                // tag.children().top() returns &[NodeHandle]
                for child_handle in tag.children().top().iter() {
                    let child_index = child_handle.get_inner() as usize;

                    // Check bounds to be safe, though tl logic implies valid index
                    if child_index < tl_nodes.len() {
                        let child_id = child_index + offset;

                        // Link child to this node
                        if let Some(me) = vdom.nodes.get_mut(my_id) {
                            me.children.push(child_id);
                        }

                        // Link parent of child to this node
                        if let Some(child) = vdom.nodes.get_mut(child_id) {
                            child.parent = Some(my_id);
                        }

                        has_parent[child_index] = true;
                    }
                }
            }
        }

        // Pass 3: Attach orphans (roots) to document root
        for (i, has_p) in has_parent.iter().enumerate() {
            if !*has_p {
                let node_id = i + offset;
                // Add to root children
                if let Some(root) = vdom.nodes.get_mut(root_id) {
                    root.children.push(node_id);
                }
                // Update node parent
                if let Some(node) = vdom.nodes.get_mut(node_id) {
                    node.parent = Some(root_id);
                }
            }
        }

        Ok(vdom)
    }
}

impl Default for HtmlParser {
    fn default() -> Self {
        Self::new()
    }
}
