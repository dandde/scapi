use crate::api::AppState;
use crate::domain::select::service::{SelectConfig, SelectService};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct SelectRequest {
    pub html: String,
    pub selector: String,
}

#[derive(Debug, Serialize)]
pub struct SelectResponse {
    pub count: usize,
    pub matches: Vec<crate::domain::select::service::SelectedElement>,
}

pub async fn select_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SelectRequest>,
) -> impl IntoResponse {
    let config = SelectConfig {
        selector: payload.selector.clone(),
    };

    // Use the smart selection logic (selects engine based on size)
    // Note: Since we are in the handler receiving a String, we have already buffered the input.
    // So "streaming" here just refers to the *engine* used (lol_html vs htmler),
    // not network streaming.
    match state.select_service.select(&payload.html, &config) {
        Ok(matches) => (
            StatusCode::OK,
            Json(SelectResponse {
                count: matches.len(),
                matches,
            }),
        )
            .into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
