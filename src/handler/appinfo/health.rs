// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use axum::response::{IntoResponse, Json};
use axum::{routing::get, Router};
use serde_json::json;

use crate::routes::route;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[utoipa::path(
    get,
    path = "/api/health",
    tag = "Information",
    responses(
        (status = 200, description = "Health check endpoint")
    )
)]
pub fn health_check() -> Router {
    async fn handler() -> impl IntoResponse {
        // let pool = crate::config::connection_pool().await;

        // FIXME: Fix database connection when build inside Docker
        // let result = sqlx::query(r#"SELECT VERSION()"#).fetch_one(&pool).await;

        // tracing::info!("Health check: {:?}", result);
        Json(json!({
          "description": "Fastrue is a user registration and authentication API",
          "name": "Fastrue",
          "version": VERSION
        }))
    }
    route("/api/health", get(handler))
}
