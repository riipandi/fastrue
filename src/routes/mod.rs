use axum::http::StatusCode;
use axum::response::IntoResponse;
// use axum::{response::Redirect, routing::get};
use axum::{routing::MethodRouter, Router};
use sqlx::{Pool, Postgres};

use crate::{handler::health, swagger, utils::error::ThrowError};

mod auth_route;
mod spa_route;

// pub use self::admin::*;
pub use self::auth_route::*;

pub fn register_routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .with_state(pool)
        // .route("/", get(|| async { Redirect::temporary("/ui") }))
        .nest_service("/api", register_auth_routes())
        .merge(swagger::register_swagger())
        .merge(health::health_check())
        .merge(spa_route::register_spa("/ui", "web"))
        .fallback(handler_404_api)
}

pub fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}

// Global 404 handler
async fn handler_404_api() -> impl IntoResponse {
    ThrowError::new(StatusCode::NOT_FOUND, "Not found")
}
