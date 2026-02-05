//! HTTP client infrastructure.

pub mod client;
pub mod streaming;

// Re-exports
pub use client::HttpClient;
pub use streaming::{ResponseStream, StreamingClient, StreamingFetchResult};
