//! Metrics collection and reporting for SCAPI.

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Metrics for a single request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMetrics {
    /// Name of the operation (e.g., "scrape", "fetch")
    pub operation: String,

    /// Total time taken in milliseconds
    pub total_ms: u128,

    /// Time spent fetching in milliseconds
    pub fetch_ms: Option<u128>,

    /// Time spent parsing in milliseconds
    pub parse_ms: Option<u128>,

    /// Time spent selecting in milliseconds
    pub select_ms: Option<u128>,

    /// Time spent extracting in milliseconds
    pub extract_ms: Option<u128>,

    /// Size of the content in bytes
    pub content_size_bytes: Option<usize>,

    /// Number of matches found
    pub matches_count: Option<usize>,

    /// Timestamp when the request started
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl RequestMetrics {
    /// Create new request metrics for an operation.
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            total_ms: 0,
            fetch_ms: None,
            parse_ms: None,
            select_ms: None,
            extract_ms: None,
            content_size_bytes: None,
            matches_count: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Set the total time from a duration.
    pub fn with_total_duration(mut self, duration: Duration) -> Self {
        self.total_ms = duration.as_millis();
        self
    }

    /// Set fetch time.
    pub fn with_fetch_time(mut self, ms: u128) -> Self {
        self.fetch_ms = Some(ms);
        self
    }

    /// Set parse time.
    pub fn with_parse_time(mut self, ms: u128) -> Self {
        self.parse_ms = Some(ms);
        self
    }

    /// Set select time.
    pub fn with_select_time(mut self, ms: u128) -> Self {
        self.select_ms = Some(ms);
        self
    }

    /// Set extract time.
    pub fn with_extract_time(mut self, ms: u128) -> Self {
        self.extract_ms = Some(ms);
        self
    }

    /// Set content size.
    pub fn with_content_size(mut self, bytes: usize) -> Self {
        self.content_size_bytes = Some(bytes);
        self
    }

    /// Set matches count.
    pub fn with_matches_count(mut self, count: usize) -> Self {
        self.matches_count = Some(count);
        self
    }
}

/// Trait for collecting metrics.
pub trait MetricsCollector: Send + Sync {
    /// Record metrics for a request.
    fn record_metrics(&self, metrics: RequestMetrics);
}

/// Simple in-memory metrics collector.
#[derive(Debug, Default)]
pub struct InMemoryMetricsCollector {
    // In a real implementation, this would use Arc<Mutex<...>> or channels
    // For now, this is a placeholder
}

impl MetricsCollector for InMemoryMetricsCollector {
    fn record_metrics(&self, metrics: RequestMetrics) {
        // For now, just log the metrics
        tracing::debug!("Metrics recorded: {:?}", metrics);
    }
}

/// Utility for timing operations.
pub struct Timer {
    start: Instant,
    operation: String,
}

impl Timer {
    /// Start timing an operation.
    pub fn start(operation: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            operation: operation.into(),
        }
    }

    /// Finish timing and return the duration.
    pub fn finish(&self) -> Duration {
        self.start.elapsed()
    }

    /// Finish timing and return milliseconds.
    pub fn finish_ms(&self) -> u128 {
        self.finish().as_millis()
    }

    /// Finish timing and update metrics.
    pub fn finish_and_update(self, metrics: &mut RequestMetrics) -> u128 {
        let ms = self.finish_ms();
        match self.operation.as_str() {
            "fetch" => metrics.fetch_ms = Some(ms),
            "parse" => metrics.parse_ms = Some(ms),
            "select" => metrics.select_ms = Some(ms),
            "extract" => metrics.extract_ms = Some(ms),
            _ => {}
        }
        ms
    }
}

/// Export metrics in Prometheus format (placeholder).
pub fn export_prometheus_metrics() -> String {
    "# TYPE scapi_requests_total counter\n".to_string()
}

/// Export metrics in JSON format (placeholder).
pub fn export_json_metrics() -> String {
    "{}".to_string()
}
