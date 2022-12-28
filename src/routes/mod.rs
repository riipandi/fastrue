use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, MethodRouter};
use axum::Router;
use axum_extra::routing::SpaRouter;
use sqlx::{Pool, Postgres};
use std::{path::PathBuf, time::Duration};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;

use crate::{
    handler::{self, health, settings},
    swagger,
    utils::error::ThrowError,
};

pub mod admin;
pub mod auth;

pub fn register_routes(pool: Pool<Postgres>) -> Router {
    // Configure logging middleware
    let trace_layer = TraceLayer::new_for_http()
        .on_request(|request: &Request<_>, _span: &Span| {
            tracing::info!("Request {} {}", request.method(), request.uri());
        })
        .on_response(|response: &Response, latency: Duration, _span: &Span| {
            tracing::info!("Response {} {:?}", response.status(), latency);
        })
        .on_failure(
            |error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                tracing::error!("Failure {} {:?}", error, latency);
            },
        );

    return Router::new()
        .with_state(pool)
        .route("/", get(|| async { Redirect::temporary("/settings") }))
        .merge(swagger::register_swagger())
        .merge(settings::settings())
        .merge(health::health_check())
        .merge(auth::register_auth_routes())
        .merge(admin::register_admin_routes())
        .merge(handler::send_email::get_send_email())
        .merge(register_spa())
        .layer(trace_layer);
}

pub fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}

// Global 404 handler
pub async fn handler_404() -> impl IntoResponse {
    ThrowError::new(StatusCode::NOT_FOUND, "Not found")
}

fn register_spa() -> SpaRouter {
    // Static SPA assets (embedded)
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("web");
    return SpaRouter::new("/spa", assets_dir);
}
