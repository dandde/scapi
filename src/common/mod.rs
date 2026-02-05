//! Common utilities and shared types for SCAPI.

pub mod error;
pub mod metrics;
pub mod types;

// Re-exports for convenience
pub use error::CommonError;
pub use metrics::{MetricsCollector, RequestMetrics};
pub use types::{ContentSize, RequestId, Timestamp};