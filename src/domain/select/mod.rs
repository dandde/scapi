//! Select operation domain logic.

pub mod config;
pub mod service;
pub mod error;
pub mod selector;
pub mod xpath;

// Re-exports
pub use config::{SelectConfig, SelectorType};
pub use service::{SelectService, DefaultSelectService};
pub use error::SelectError;
pub use selector::{Selector, CssSelector, SelectorStrategy};
pub use xpath::XPathPattern;