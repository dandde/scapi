//! Fetch service implementation.

use crate::common::metrics::Timer;
use crate::infra::http::HttpClient;

use super::config::FetchConfig;
use super::error::FetchError;
use chrono::{DateTime, Utc};

/// Result of a fetch operation.
#[derive(Debug)]
pub struct FetchResult {
    /// HTML content
    pub content: String,
    /// Content length in bytes
    pub length: usize,
    /// HTTP status code
    pub status_code: u16,
    /// Final URL after redirects
    pub final_url: String,
    /// Timestamp when the fetch completed
    pub timestamp: DateTime<Utc>,
}

/// Trait for fetch services.
pub trait FetchService: Send + Sync {
    /// Fetch HTML content from a URL.
    fn fetch(
        &self,
        url: &str,
        config: &FetchConfig,
    ) -> impl std::future::Future<Output = Result<FetchResult, FetchError>> + Send;
}

/// Default implementation of the fetch service.
pub struct DefaultFetchService {
    /// HTTP client
    pub client: HttpClient,
}

impl DefaultFetchService {
    /// Create a new fetch service with the given HTTP client.
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }
}

impl FetchService for DefaultFetchService {
    fn fetch(
        &self,
        url: &str,
        config: &FetchConfig,
    ) -> impl std::future::Future<Output = Result<FetchResult, FetchError>> + Send {
        let client = self.client.clone();
        let url = url.to_string();
        let config = config.clone();

        async move {
            // Start timing the operation
            let timer = Timer::start("fetch");

            // Perform fetch operation
            let content = client.fetch(&url, &config).await?;

            // Build result
            let result = FetchResult {
                content: content.clone(),
                length: content.len(),
                // TODO: Get actual status code and final URL from client response if we modify client to return full response
                // For now, assuming success since client.fetch returns content string on success
                status_code: 200,
                final_url: url,
                timestamp: Utc::now(),
            };

            // Log completion
            tracing::info!(
                "Fetched {} bytes from {} in {}ms",
                result.length,
                result.final_url,
                timer.finish_ms()
            );

            Ok(result)
        }
    }
}
