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
        }
    }
}