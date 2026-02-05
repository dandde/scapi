//! Fetch service implementation.

use crate::common::metrics::Timer;

use crate::infra::http::HttpClient;
use crate::infra::http::streaming::ResponseStream;

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

/// Result of a streaming fetch operation.
pub struct StreamingFetchResult {
    /// Content stream
    // Using simple formatting for debug since ResponseStream might not implement Debug or is complex
    #[allow(dead_code)]
    pub stream: ResponseStream,
    /// HTTP status code
    pub status_code: u16,
    /// Final URL after redirects
    pub final_url: String,
    /// Timestamp when the fetch started/headers received
    pub timestamp: DateTime<Utc>,
    /// Content length if known
    pub content_length: Option<u64>,
}

impl std::fmt::Debug for StreamingFetchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamingFetchResult")
            .field("status_code", &self.status_code)
            .field("final_url", &self.final_url)
            .field("timestamp", &self.timestamp)
            .field("content_length", &self.content_length)
            .finish()
    }
}

/// Trait for fetch services.
pub trait FetchService: Send + Sync {
    /// Fetch HTML content from a URL.
    fn fetch(
        &self,
        url: &str,
        config: &FetchConfig,
    ) -> impl std::future::Future<Output = Result<FetchResult, FetchError>> + Send;

    /// Fetch HTML content as a stream.
    fn fetch_stream(
        &self,
        url: &str,
        config: &FetchConfig,
    ) -> impl std::future::Future<Output = Result<StreamingFetchResult, FetchError>> + Send;
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

    fn fetch_stream(
        &self,
        url: &str,
        config: &FetchConfig,
    ) -> impl std::future::Future<Output = Result<StreamingFetchResult, FetchError>> + Send {
        let client = self.client.clone();
        let url = url.to_string();
        let config = config.clone();

        async move {
            let timer = Timer::start("fetch_stream");

            // Perform streaming fetch
            let result = client.streaming().fetch_stream(&url, &config).await?;

            let domain_result = StreamingFetchResult {
                stream: result.stream,
                status_code: result.status_code,
                final_url: result.final_url.clone(),
                timestamp: Utc::now(),
                content_length: result.content_length,
            };

            tracing::info!(
                "Started streaming from {} (status {}) in {}ms",
                domain_result.final_url,
                domain_result.status_code,
                timer.finish_ms()
            );

            Ok(domain_result)
        }
    }
}
