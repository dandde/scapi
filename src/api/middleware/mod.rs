//! HTTP middleware for SCAPI.

pub mod request_id;
pub mod metrics;
pub mod error_handler;

// Re-export middleware constructors
pub use request_id::request_id_middleware;
pub use metrics::metrics_middleware;
pub use error_handler::error_handler_middleware;