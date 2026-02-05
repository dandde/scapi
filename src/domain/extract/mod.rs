//! Extract operation domain logic.

pub mod config;
pub mod service;
pub mod error;
pub mod rules;

// Re-exports
pub use config::ExtractConfig;
pub use service::{ExtractService, DefaultExtractService};
pub use error::ExtractError;
pub use rules::ExtractionRule;