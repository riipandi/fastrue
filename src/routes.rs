// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, get_service, MethodRouter};
use axum::{http::StatusCode, Router};
use std::{io, path::PathBuf};
use tower_http::services::{ServeDir, ServeFile};

use crate::config::get_envar;
use crate::handler::{admin, appinfo, auth, user};
use crate::state::AppState;
use crate::{swagger, utils::error::ThrowError};

pub fn register_routes(state: AppState) -> Router {
    Router::new()
        .with_state(state.clone())
        .route("/", get(|| async { Redirect::temporary("/ui") }))
        .nest_service("/api", register_api_routes())
        .merge(swagger::register_swagger())
        .merge(register_spa("/ui", "web"))
        .fallback(handler_404_api)
}

fn register_api_routes() -> Router {
    Router::new()
        .merge(admin::get_all_users())
        .merge(admin::invite_by_admin())
        .merge(appinfo::health_check())
        .merge(appinfo::settings())
        .merge(auth::logout())
        .merge(auth::recover())
        .merge(auth::signup())
        .merge(auth::token())
        .merge(auth::verify())
        .merge(user::get_user())
        .merge(user::put_user())
}

pub fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}

// Static SPA assets (embedded)`ServeDir` allows setting a fallback if an asset is not found.
// So with this `GET /assets/doesnt-exist.ext` will return `index.html` rather than a 404.
// Reference: https://github.com/tokio-rs/axum/blob/main/examples/static-file-server/src/main.rs#L67
fn register_spa(path: &str, dir: &str) -> Router {
    // Disable SPA user interface when HEADLESS_MODE = true
    let headless_mode = get_envar("FASTRUE_HEADLESS_MODE", Some("false"));
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

// Global 404 handler for SPA
async fn handler_404_spa(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

// Global 404 handler for API
async fn handler_404_api() -> impl IntoResponse {
    ThrowError::new(StatusCode::NOT_FOUND, "Not found")
}
