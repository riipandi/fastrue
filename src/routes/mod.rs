use axum::http::StatusCode;
use axum::response::IntoResponse;
// use axum::{response::Redirect, routing::get};
use axum::{
    routing::{get_service, MethodRouter},
    Router,
};
use sqlx::{Pool, Postgres};
use std::{io, path::PathBuf};
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    handler::{health, settings},
    swagger,
    utils::error::ThrowError,
};

mod admin;
mod auth;

pub use self::admin::*;
pub use self::auth::*;

pub fn register_routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .with_state(pool)
        // .route("/", get(|| async { Redirect::temporary("/settings") }))
        .merge(swagger::register_swagger())
        .merge(settings::settings())
        .merge(health::health_check())
        .merge(register_auth_routes())
        .merge(register_admin_routes())
        .merge(register_spa())
}

pub fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}

// Global 404 handler
pub async fn handler_404_api() -> impl IntoResponse {
    ThrowError::new(StatusCode::NOT_FOUND, "Not found")
}

// Static SPA assets (embedded)
// `ServeDir` allows setting a fallback if an asset is not found. So with this
// `GET /assets/doesnt-exist.ext` will return `index.html` rather than a 404.
// Reference: https://github.com/tokio-rs/axum/blob/main/examples/static-file-server/src/main.rs#L67
fn register_spa() -> Router {
    let spa_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("web");
    let spa_index = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("web/index.html");
    let serve_dir = ServeDir::new(spa_dir).not_found_service(ServeFile::new(spa_index));
    let serve_dir = get_service(serve_dir).handle_error(handler_404_spa);

    Router::new()
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir)
}

async fn handler_404_spa(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
