//! Global error handling middleware.

use axum::{extract::Request, middleware::Next, response::Response};

/// Error handler middleware that catches panics and converts them to proper error responses.
pub async fn error_handler_middleware(request: Request, next: Next) -> Response {
    // In a real implementation, this would catch panics and convert them to 500 errors
    // For now, just pass through
    next.run(request).await
}
