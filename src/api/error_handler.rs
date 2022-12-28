use axum::http::{Method, StatusCode, Uri};
use axum::response::{Html, IntoResponse};

use std::io;
// use std::path::PathBuf;

// Global 404 handler
pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

pub async fn handler_spa_error(_method: Method, _uri: Uri, _err: io::Error) -> Html<&'static str> {
    // let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("web");
    // format!("{} {} failed with {}", method, uri, err)
    Html("<p>All is well!</p>")
}
