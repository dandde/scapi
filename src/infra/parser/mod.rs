//! HTML parser infrastructure.

pub mod vdom;
pub mod html;

// Re-exports
pub use vdom::VDom;
pub use html::HtmlParser;

/// Trait for parser backends.
pub trait ParserBackend: Send + Sync {
    /// Parse HTML into a VDOM.
    fn parse(&self, html: &str) -> Result<VDom, crate::domain::parse::error::ParseError>;
}