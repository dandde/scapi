//! Common type definitions for SCAPI.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request ID wrapper for tracking requests across the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestId(String);

impl RequestId {
    /// Create a new random request ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Create a request ID from a string.
    pub fn from_string(id: String) -> Self {
        Self(id)
    }

    /// Get the request ID as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get the request ID as a string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Timestamp wrapper using UTC timezone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    /// Create a new timestamp for the current time.
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Create a timestamp from a DateTime.
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    /// Get the inner DateTime.
    pub fn as_datetime(&self) -> &DateTime<Utc> {
        &self.0
    }

    /// Convert to RFC3339 string.
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Content size wrapper with validation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ContentSize(usize);

impl ContentSize {
    /// Create a new content size with validation.
    pub fn new(size: usize) -> Result<Self, String> {
        if size == 0 {
            Err("Content size cannot be zero".to_string())
        } else {
            Ok(Self(size))
        }
    }

    /// Create a content size without validation (for trusted sources).
    pub fn new_unchecked(size: usize) -> Self {
        Self(size)
    }

    /// Get the size in bytes.
    pub fn as_bytes(&self) -> usize {
        self.0
    }

    /// Get the size in kilobytes.
    pub fn as_kb(&self) -> f64 {
        self.0 as f64 / 1024.0
    }

    /// Get the size in megabytes.
    pub fn as_mb(&self) -> f64 {
        self.as_kb() / 1024.0
    }
}

impl From<usize> for ContentSize {
    fn from(size: usize) -> Self {
        Self::new_unchecked(size)
    }
}

impl From<ContentSize> for usize {
    fn from(val: ContentSize) -> Self {
        val.0
    }
}

impl std::fmt::Display for ContentSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} bytes", self.0)
    }
}