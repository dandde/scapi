//! HTTP client wrapper.

use reqwest::{Client, Response};
use std::time::Duration;

use crate::domain::fetch::config::FetchConfig;
use crate::domain::fetch::error::FetchError;

use crate::infra::http::streaming::StreamingClient;

/// HTTP client wrapper.
#[derive(Debug, Clone)]
pub struct HttpClient {
    /// Inner reqwest client
    client: Client,
    streaming_client: StreamingClient,
}

impl HttpClient {
    /// Create a new HTTP client with default configuration.
    pub fn new() -> Result<Self, FetchError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| FetchError::NetworkError(e.to_string()))?;

        let streaming_client = StreamingClient::new(100 * 1024 * 1024)?; // 100MB default

        Ok(Self {
            client,
            streaming_client,
        })
    }

    /// Create a new HTTP client with custom configuration.
    pub fn with_config(config: &FetchConfig) -> Result<Self, FetchError> {
        let mut builder = Client::builder()
            .timeout(config.timeout)
            .connect_timeout(config.connect_timeout)
            .user_agent(&config.user_agent);

        if !config.verify_tls {
            builder = builder.danger_accept_invalid_certs(true);
        }

        if config.follow_redirects {
            builder = builder.redirect(reqwest::redirect::Policy::limited(config.max_redirects));
        } else {
            builder = builder.redirect(reqwest::redirect::Policy::none());
        }

        let client = builder
            .build()
            .map_err(|e| FetchError::NetworkError(e.to_string()))?;

        let streaming_client = StreamingClient::new(config.max_content_size)?;

        Ok(Self {
            client,
            streaming_client,
        })
    }

    /// Fetch content from a URL.
    pub async fn fetch(&self, url: &str, config: &FetchConfig) -> Result<String, FetchError> {
        // Use streaming client for all fetches to enforce size limits
        let (content, _metadata) = self
            .streaming_client
            .fetch_to_string(url, config, config.max_content_size)
            .await?;

        Ok(content)
    }

    /// Perform a GET request.
    pub async fn get(&self, url: &str) -> Result<Response, FetchError> {
        self.client
            .get(url)
            .send()
            .await
            .map_err(|e| FetchError::NetworkError(e.to_string()))
    }

    /// Perform a POST request.
    pub async fn post(&self, url: &str, body: &str) -> Result<Response, FetchError> {
        self.client
            .post(url)
            .body(body.to_string())
            .send()
            .await
            .map_err(|e| FetchError::NetworkError(e.to_string()))
    }

    /// Get streaming client for manual control
    pub fn streaming(&self) -> &StreamingClient {
        &self.streaming_client
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}
