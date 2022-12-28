use axum::{
    http::Request,
    response::{Redirect, Response},
    routing::get,
    Router,
};
use axum_extra::routing::SpaRouter;
use sqlx::{Pool, Postgres};
use std::{path::PathBuf, time::Duration};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{api, swagger};

pub fn register_router(pool: Pool<Postgres>) -> Router {
    // Static SPA assets (embedded)
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("web");
    let spa =
        SpaRouter::new("/spa", assets_dir).handle_error(api::error_handler::handler_spa_error);

    return Router::new()
        .merge(SwaggerUi::new("/swagger").url("/swagger/openapi.json", swagger::ApiDoc::openapi()))
        .route("/", get(|| async { Redirect::temporary("/spa") }))
        .with_state(pool)
        .merge(api::root::hello())
        .merge(api::root::health_check())
        .merge(api::send_email::get_send_email())
        .merge(spa)
        .layer(
            TraceLayer::new_for_http()
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
                ),
        );
}
