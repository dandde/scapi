use super::error::SelectError;
use crate::infra::parser::htmler_adapter::HtmlerAdapter;
use crate::infra::parser::streaming_adapter::StreamingAdapter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectedElement {
    pub element_id: usize,
    pub tag: String,
    pub text: Option<String>,
    pub attributes: HashMap<String, String>,
    pub html: String,
}

#[derive(Debug, Clone)]
pub struct SelectConfig {
    pub selector: String,
    // Add other config options (first_only, etc) later
}

pub trait SelectService: Send + Sync {
    fn select(
        &self,
        html: &str,
        config: &SelectConfig,
    ) -> Result<Vec<SelectedElement>, SelectError>;
}

#[derive(Clone)]
pub struct DefaultSelectService {
    streaming_threshold_bytes: usize,
}

impl DefaultSelectService {
    pub fn new(streaming_threshold_bytes: usize) -> Self {
        Self {
            streaming_threshold_bytes,
        }
    }

    fn select_buffered(
        &self,
        html: &str,
        selector: &str,
    ) -> Result<Vec<SelectedElement>, SelectError> {
        HtmlerAdapter::select(html, selector)
    }

    fn select_streaming(
        &self,
        html: &str,
        selector: &str,
    ) -> Result<Vec<SelectedElement>, SelectError> {
        StreamingAdapter::select_from_string(html, selector)
    }
}

// Default to 1MB threshold
impl Default for DefaultSelectService {
    fn default() -> Self {
        Self::new(1024 * 1024)
    }
}

impl SelectService for DefaultSelectService {
    fn select(
        &self,
        html: &str,
        config: &SelectConfig,
    ) -> Result<Vec<SelectedElement>, SelectError> {
        if html.len() > self.streaming_threshold_bytes {
            tracing::info!(
                "Input size {} > threshold {}. Using Streaming Engine (lol_html).",
                html.len(),
                self.streaming_threshold_bytes
            );
            self.select_streaming(html, &config.selector)
        } else {
            tracing::info!(
                "Input size {} <= threshold {}. Using Buffered Engine (HTMLer).",
                html.len(),
                self.streaming_threshold_bytes
            );
            self.select_buffered(html, &config.selector)
        }
    }
}
