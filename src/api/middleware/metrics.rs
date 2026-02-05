//! Metrics collection middleware.

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;

/// Metrics middleware that collects timing information.
pub async fn metrics_middleware(
    request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();

    // Process the request
    let response = next.run(request).await;

    // Calculate duration
    let duration = start.elapsed();

    // Log metrics (in a real implementation, this would send to a metrics collector)
    tracing::debug!(
        "Request processed in {:?}",
        duration
    );

    response
}