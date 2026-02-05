//! Extract structured data handler.

use axum::{extract::State, response::Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;
use crate::common::error::CommonError;
use crate::domain::extract::config::ExtractConfig;
use crate::domain::extract::rules::ExtractionRule;
use crate::domain::extract::service::{ExtractResult, ExtractService};

/// Extract request payload.
#[derive(Debug, Deserialize)]
pub struct ExtractRequest {
    /// HTML content to extract from
    pub html: String,
    /// Extraction rules
    pub rules: Vec<ExtractionRule>,
    /// Configuration options
    #[serde(default)]
    pub config: ExtractConfig,
}

/// Extract response payload.
#[derive(Debug, Serialize)]
pub struct ExtractResponse {
    /// Request ID
    pub id: String,
    /// Timestamp
    pub timestamp: String,
    /// Extracted result
    pub result: ExtractResult,
}

/// Extract structured data from HTML.
pub async fn extract_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ExtractRequest>,
) -> Result<Json<ExtractResponse>, CommonError> {
    let result = state
        .extract_service
        .extract(&request.html, &request.rules, &request.config)
        .await
        .map_err(|e| CommonError::internal(e.to_string()))?;

    Ok(Json(ExtractResponse {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        result,
    }))
}
