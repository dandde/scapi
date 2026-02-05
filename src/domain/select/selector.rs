//! Selector parsing and execution.

use crate::infra::parser::VDom;

/// Selector type.
#[derive(Debug, Clone)]
pub struct Selector {
    /// Selector string
    pub selector_str: String,
    /// Selector type
    pub selector_type: super::config::SelectorType,
}

/// Trait for selector strategies.
pub trait SelectorStrategy: Send + Sync {
    /// Parse a selector string.
    fn parse(input: &str) -> Result<Self, super::error::SelectError>
    where
        Self: Sized;

    /// Select elements from a VDOM.
    fn select(&self, vdom: &VDom) -> Vec<usize>;
}

/// CSS selector implementation.
#[derive(Debug, Clone)]
pub struct CssSelector {
    /// CSS selector string
    pub selector: String,
}

impl SelectorStrategy for CssSelector {
    fn parse(input: &str) -> Result<Self, super::error::SelectError> {
        // TODO: Validate CSS selector syntax
        Ok(Self {
            selector: input.to_string(),
        })
    }

    fn select(&self, _vdom: &VDom) -> Vec<usize> {
        // TODO: Implement CSS selector execution
        vec![]
    }
}

/// XPath selector implementation (placeholder).
#[derive(Debug, Clone)]
pub struct XPathSelector {
    /// XPath expression
    pub expression: String,
}

impl SelectorStrategy for XPathSelector {
    fn parse(input: &str) -> Result<Self, super::error::SelectError> {
        // TODO: Validate XPath syntax
        Ok(Self {
            expression: input.to_string(),
        })
    }

    fn select(&self, _vdom: &VDom) -> Vec<usize> {
        // TODO: Implement XPath selector execution
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_selector_parse() {
        let selector = CssSelector::parse("div.article").unwrap();
        assert_eq!(selector.selector, "div.article");
    }

    #[test]
    fn test_css_selector_parse_empty() {
        let selector = CssSelector::parse("");
        assert!(selector.is_ok());
    }

    #[test]
    fn test_xpath_selector_parse() {
        let selector = XPathSelector::parse("//div[@class='article']").unwrap();
        assert_eq!(selector.expression, "//div[@class='article']");
    }
}