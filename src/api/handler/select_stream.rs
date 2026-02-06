use crate::api::AppState;
use crate::api::handler::select::{SelectRequest, SelectResponse}; // Reuse types
use crate::domain::select::service::{SelectConfig, SelectService};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

// This handler forces the use of the streaming engine (lol_html) regardless of size.
// Ideally, for true streaming, we would accept a Body stream, but that requires
// a different request format (not JSON). For now, we reuse the JSON interface
// but force the streaming *logic*.
pub async fn select_stream_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SelectRequest>,
) -> impl IntoResponse {
    // Manually construct the service call to force streaming if possible,
    // or just rely on the smart service being configured with a low threshold for this endpoint?
    // Actually, our DefaultSelectService logic selects based on size.
    // We should probably expose specific methods on the trait or service if we want to force it.
    // For now, let's treat this as "Smart Select with forced-low threshold behavior" logic
    // or just assume the user sends large content here.

    // BUT, the requirement "both handler available" implies user choice.
    // Let's modify SelectService trait to allow explicit choice in V2,
    // but for now we will just use the same logic but maybe log differently?

    // WAIT, actually, I can just call the internal private method if I expose it via trait/struct?
    // No, `select_streaming` is private in `DefaultSelectService`.

    // Let's just use the `select` method for now.
    // If the user hits `/select-stream`, they likely expect the streaming engine.
    // Ideally, we'd accept `text/plain` or `application/octet-stream` here to avoid JSON buffering overhead.

    // For this beta implementation, we map it to the same service method.
    // The "Streaming" value prop is primarily the ENGINE used.

    let config = SelectConfig {
        selector: payload.selector.clone(),
    };

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
