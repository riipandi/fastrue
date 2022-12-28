use axum::{
    http::Request,
    response::{Redirect, Response},
    routing::{get, MethodRouter},
    Router,
};
use axum_extra::routing::SpaRouter;
use sqlx::{Pool, Postgres};
use std::{path::PathBuf, time::Duration};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;

use crate::{handler, swagger};

pub fn register_router(pool: Pool<Postgres>) -> Router {
    // Static SPA assets (embedded)
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("web");
    let spa =
        SpaRouter::new("/spa", assets_dir).handle_error(handler::error_handler::handler_spa_error);

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
        .route("/", get(|| async { Redirect::temporary("/spa") }))
        .merge(swagger::register_swagger())
        .merge(handler::root::hello())
        .merge(handler::root::health_check())
        .merge(handler::send_email::get_send_email())
        .merge(spa)
        .layer(trace_layer);
}

pub fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}
