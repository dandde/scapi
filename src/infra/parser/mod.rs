//! HTML parser infrastructure.

pub mod html;
pub mod htmler_adapter;
pub mod streaming_adapter;
pub mod vdom;

// Re-exports
pub use html::HtmlParser;
pub use vdom::VDom;

/// Trait for parser backends.
pub trait ParserBackend: Send + Sync {
    /// Parse HTML into a VDOM.
    fn parse(&self, html: &str) -> Result<VDom, crate::domain::parse::error::ParseError>;
}
