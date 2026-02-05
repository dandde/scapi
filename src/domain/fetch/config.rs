//! Configuration for fetch operations.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for fetch operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchConfig {
    /// Request timeout duration
    pub timeout: Duration,
    /// User agent string
    pub user_agent: String,
    /// Whether to follow redirects
    pub follow_redirects: bool,
    /// Maximum number of redirects to follow
    pub max_redirects: usize,
    /// Whether to verify TLS certificates
    pub verify_tls: bool,
    /// Connection timeout duration
    pub connect_timeout: Duration,
    /// Read timeout duration
    pub read_timeout: Duration,
    /// Maximum content size in bytes
    #[serde(default = "default_max_content_size")]
    pub max_content_size: usize,
    /// Use streaming for files larger than this
    #[serde(default = "default_streaming_threshold")]
    pub streaming_threshold: usize,
    /// Buffer size for streaming operations
    #[serde(default = "default_stream_buffer_size")]
    pub stream_buffer_size: usize,
}

fn default_max_content_size() -> usize {
    100 * 1024 * 1024 // 100MB
}

fn default_streaming_threshold() -> usize {
    5 * 1024 * 1024 // 5MB
}

fn default_stream_buffer_size() -> usize {
    64 * 1024 // 64KB
}

impl Default for FetchConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            user_agent: "SCAPI/1.0".to_string(),
            follow_redirects: true,
            max_redirects: 5,
            verify_tls: true,
            connect_timeout: Duration::from_secs(10),
            read_timeout: Duration::from_secs(30),
            max_content_size: default_max_content_size(),
            streaming_threshold: default_streaming_threshold(),
            stream_buffer_size: default_stream_buffer_size(),
        }
    }
}
