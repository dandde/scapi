//! API Layer for SCAPI.
//!
//! This module contains HTTP handlers, request/response models, and middleware.

pub mod handler;
pub mod middleware;
pub mod model;

use axum::Router;
use axum::routing::post;
use std::sync::Arc;

use crate::AppState;

/// Create the main router for the SCAPI application.
///
/// This function sets up all routes and applies middleware layers.
pub fn create_router(state: AppState) -> Router {
    // Create router with health endpoint (other endpoints will be added later)
    Router::new()
        //.route("/health", get(handler::health::health_handler)) // TODO: Implement health
        .route("/api/v1/fetch", post(handler::fetch::fetch_handler))
        .route("/api/v1/parse", post(handler::parse::parse_handler))
        .with_state(Arc::new(state))
    // Middleware layers will be added when middleware is implemented
    // .layer(middleware::request_id())
    // .layer(middleware::metrics())
    // .layer(middleware::error_handler())
}
