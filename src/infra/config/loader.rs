//! Configuration loader.

use std::time::Duration;

use crate::common::error::CommonError;
use crate::domain::extract::config::ExtractConfig;
use crate::domain::fetch::config::FetchConfig;
use crate::domain::parse::config::ParseConfig;

/// Server configuration.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Port to listen on
    pub port: u16,
    /// Bind address
    pub bind_addr: String,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Request timeout
    pub request_timeout: Duration,
}

/// Application configuration.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Server configuration
    pub server: ServerConfig,
    /// Fetch configuration
    pub fetch: FetchConfig,
    /// Parse configuration
    pub parse: ParseConfig,

    /// Extract configuration
    pub extract: ExtractConfig,
}

impl AppConfig {
    /// Load configuration from environment variables.
    pub fn from_env() -> Result<Self, CommonError> {
        dotenv::dotenv().ok();

        let server = ServerConfig {
            port: std::env::var("SCAPI_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|e| CommonError::config(format!("Invalid SCAPI_PORT: {}", e)))?,
            bind_addr: std::env::var("SCAPI_BIND_ADDR").unwrap_or_else(|_| "0.0.0.0".to_string()),
            max_connections: std::env::var("SCAPI_MAX_CONCURRENT_REQUESTS")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()
                .map_err(|e| {
                    CommonError::config(format!("Invalid SCAPI_MAX_CONCURRENT_REQUESTS: {}", e))
                })?,
            request_timeout: Duration::from_secs(
                std::env::var("SCAPI_REQUEST_TIMEOUT_SECS")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .map_err(|e| {
                        CommonError::config(format!("Invalid SCAPI_REQUEST_TIMEOUT_SECS: {}", e))
                    })?,
            ),
        };

        let fetch = FetchConfig {
            timeout: Duration::from_secs(
                std::env::var("SCAPI_FETCH_TIMEOUT_SECS")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .map_err(|e| {
                        CommonError::config(format!("Invalid SCAPI_FETCH_TIMEOUT_SECS: {}", e))
                    })?,
            ),
            user_agent: std::env::var("SCAPI_FETCH_USER_AGENT")
                .unwrap_or_else(|_| "SCAPI/1.0".to_string()),
            follow_redirects: std::env::var("SCAPI_FETCH_FOLLOW_REDIRECTS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            max_redirects: std::env::var("SCAPI_FETCH_MAX_REDIRECTS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            verify_tls: std::env::var("SCAPI_FETCH_VERIFY_TLS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            connect_timeout: Duration::from_secs(
                std::env::var("SCAPI_FETCH_CONNECT_TIMEOUT_SECS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
            ),
            read_timeout: Duration::from_secs(
                std::env::var("SCAPI_FETCH_READ_TIMEOUT_SECS")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
            ),
            max_content_size: std::env::var("SCAPI_FETCH_MAX_CONTENT_SIZE")
                .unwrap_or_else(|_| "104857600".to_string())
                .parse()
                .unwrap_or(104857600),
            streaming_threshold: std::env::var("SCAPI_FETCH_STREAMING_THRESHOLD")
                .unwrap_or_else(|_| "5242880".to_string())
                .parse()
                .unwrap_or(5242880),
            stream_buffer_size: std::env::var("SCAPI_FETCH_STREAM_BUFFER_SIZE")
                .unwrap_or_else(|_| "65536".to_string())
                .parse()
                .unwrap_or(65536),
        };

        let parse = ParseConfig {
            detect_encoding: std::env::var("SCAPI_PARSE_DETECT_ENCODING")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            handle_malformed: std::env::var("SCAPI_PARSE_HANDLE_MALFORMED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            max_size_bytes: std::env::var("SCAPI_PARSE_MAX_SIZE_BYTES")
                .unwrap_or_else(|_| "104857600".to_string())
                .parse()
                .unwrap_or(104857600),
            extract_attributes: std::env::var("SCAPI_PARSE_EXTRACT_ATTRIBUTES")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            include_hierarchy: std::env::var("SCAPI_PARSE_INCLUDE_HIERARCHY")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        };

        let extract = ExtractConfig {
            trim_whitespace: std::env::var("SCAPI_EXTRACT_TRIM_WHITESPACE")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            decode_html_entities: std::env::var("SCAPI_EXTRACT_DECODE_HTML_ENTITIES")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            max_fields: std::env::var("SCAPI_EXTRACT_MAX_FIELDS")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .unwrap_or(100),
            validate_types: std::env::var("SCAPI_EXTRACT_VALIDATE_TYPES")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            default_value: std::env::var("SCAPI_EXTRACT_DEFAULT_VALUE").ok(),
            strict_mode: std::env::var("SCAPI_EXTRACT_STRICT_MODE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        };

        Ok(Self {
            server,
            fetch,
            parse,

            extract,
        })
    }
}
