//! SCAPI - Scalable CSS/XPath API
//!
//! A production-ready HTTP API server for atomic web scraping operations.
//!
//! # Overview
//!
//! SCAPI provides 4 atomic operations for web scraping:
//! 1. **FETCH** - Download HTML from URLs
//! 2. **PARSE** - Build queryable DOM structure
//! 3. **SELECT** - Find elements via CSS/XPath
//! 4. **EXTRACT** - Transform data with validation
//!
//! # Architecture
//!
//! The system is organized in 3 layers:
//! - **API Layer** - HTTP handlers, request/response models, middleware
//! - **Domain Layer** - Business logic (the 4 operations)
//! - **Infrastructure Layer** - External integrations (HTTP client, HTML parser, logging)
//!
//! # Quick Start
//!
//! ```no_run
//! use scapi::api;
//! use scapi::AppState;
//! use scapi::infra::config;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load configuration
//!     let config = config::AppConfig::from_env()?;
//!
//!     // Create application state
//!     let state = AppState::new()?;
//!
//!     // Create router
//!     let app = api::create_router(state);
//!
//!     // Start server
//!     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
//!     axum::serve(listener, app).await?;
//!
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod common;
pub mod domain;
pub mod infra;

// Re-exports for common types
pub use common::error::CommonError;
pub use common::metrics::{MetricsCollector, RequestMetrics};
pub use common::types::{ContentSize, RequestId, Timestamp};

// Application state
#[derive(Clone)]
pub struct AppState {
    /// Fetch service
    pub fetch_service: std::sync::Arc<domain::fetch::service::DefaultFetchService>,
    /// Parse service
    pub parse_service: std::sync::Arc<domain::parse::service::DefaultParseService>,

    /// Extract service
    pub extract_service: std::sync::Arc<domain::extract::service::DefaultExtractService>,
}

impl AppState {
    /// Create a new application state.
    pub fn new() -> Result<Self, CommonError> {
        let http_client = infra::http::HttpClient::new()
            .map_err(|e| CommonError::config(format!("Failed to create HTTP client: {}", e)))?;

        let fetch_service = std::sync::Arc::new(domain::fetch::service::DefaultFetchService::new(
            http_client,
        ));
        let parse_service = std::sync::Arc::new(domain::parse::service::DefaultParseService::new());

        let extract_service = std::sync::Arc::new(
            domain::extract::service::DefaultExtractService::new(parse_service.clone()),
        );

        Ok(Self {
            fetch_service,
            parse_service,

            extract_service,
        })
    }
}
