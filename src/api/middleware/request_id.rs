//! Request ID middleware.

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use tower_http::request_id::{MakeRequestId, RequestId};
use uuid::Uuid;

/// Request ID maker that generates UUIDs.
#[derive(Clone, Default)]
pub struct UuidRequestIdMaker;

impl MakeRequestId for UuidRequestIdMaker {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        Some(RequestId::new(Uuid::new_v4().to_string().parse().unwrap()))
    }
}

/// Request ID middleware that adds a request ID to each request.
pub async fn request_id_middleware(
    request: Request,
    next: Next,
) -> Response {
    // The tower_http::request_id middleware will be added as a layer
    // This is a simple async middleware that just passes through
    next.run(request).await
}