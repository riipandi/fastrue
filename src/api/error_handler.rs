use axum::http::{Method, StatusCode, Uri};
use axum::response::IntoResponse;

use std::io;

// Global 404 handler
pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

pub async fn handler_spa_error(method: Method, uri: Uri, err: io::Error) -> String {
    format!("{} {} failed with {}", method, uri, err)
}
