use axum::http::{Method, StatusCode, Uri};
use axum::response::IntoResponse;
use std::io;

use crate::utils::throw_error::ThrowError;

// Global 404 handler
pub async fn handler_404() -> impl IntoResponse {
    ThrowError::new(StatusCode::NOT_FOUND, "Not found")
}

pub async fn handler_spa_error(method: Method, uri: Uri, err: io::Error) {
    format!("{} {} failed with {}", method, uri, err);
}
