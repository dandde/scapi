//! HTTP client wrapper.

use reqwest::{Client, Response};
use std::time::Duration;

use crate::domain::fetch::config::FetchConfig;
use crate::domain::fetch::error::FetchError;

/// HTTP client wrapper.
#[derive(Debug, Clone)]
pub struct HttpClient {
    /// Inner reqwest client
    client: Client,
}

impl HttpClient {
    /// Create a new HTTP client with default configuration.
    pub fn new() -> Result<Self, FetchError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| FetchError::NetworkError(e.to_string()))?;

        Ok(Self { client })
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

        Ok(Self { client })
    }

    /// Fetch content from a URL.
    pub async fn fetch(&self, url: &str, config: &FetchConfig) -> Result<String, FetchError> {
        let response = self
            .client
            .get(url)
            .timeout(config.timeout)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    FetchError::Timeout(e.to_string())
                } else {
                    FetchError::NetworkError(e.to_string())
                }
            })?;

        let status = response.status();
        let final_url = response.url().to_string();

        if !status.is_success() {
            return Err(FetchError::ServerError(format!(
                "HTTP {} for {}",
                status, final_url
            )));
        }

        let content = response
            .text()
            .await
            .map_err(|e| FetchError::NetworkError(e.to_string()))?;

        // Check content size limit
        let _content_size = content.len();
        // TODO: Check against config.max_size

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
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}
