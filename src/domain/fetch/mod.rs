//! Fetch operation domain logic.

pub mod config;
pub mod service;
pub mod error;

// Re-exports
pub use config::FetchConfig;
pub use service::{FetchService, DefaultFetchService};
pub use error::FetchError;