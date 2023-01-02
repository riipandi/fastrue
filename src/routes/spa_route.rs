use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router};
use std::{io, path::PathBuf};
use tower_http::services::{ServeDir, ServeFile};

use crate::config::get_envar;

use super::handler_404_api;

// Static SPA assets (embedded)`ServeDir` allows setting a fallback if an asset is not found.
// So with this `GET /assets/doesnt-exist.ext` will return `index.html` rather than a 404.
// Reference: https://github.com/tokio-rs/axum/blob/main/examples/static-file-server/src/main.rs#L67
pub fn register_spa(path: &str, dir: &str) -> Router {
    // Disable SPA user interface when HEADLESS_MODE = true
    let headless_mode = get_envar("TRUSTY_HEADLESS_MODE", Some("false"));
    if headless_mode.trim().parse().unwrap() {
        Router::new().fallback(handler_404_api)
    } else {
        let spa_index_file = [dir, "index.html"].join("/");
        let spa_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(dir);
        let spa_index = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(spa_index_file);
        let serve_dir = ServeDir::new(spa_dir).not_found_service(ServeFile::new(spa_index));
        let serve_dir = get_service(serve_dir).handle_error(handler_404_spa);

        Router::new()
            .nest_service(path, serve_dir.clone())
            .fallback_service(serve_dir)
    }
}

async fn handler_404_spa(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
