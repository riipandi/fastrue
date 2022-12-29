use axum::response::{IntoResponse, Json};
use axum::{routing::get, Router};
use serde_json::json;

use crate::routes::route;

#[utoipa::path(
    get,
    path = "/health",
    tag = "Information",
    responses(
        (status = 200, description = "Health check endpoint")
    )
)]
pub fn health_check() -> Router {
    async fn handler() -> impl IntoResponse {
        // FIXME: Fix database connection when build inside Docker
        let pool = crate::config::connection_pool().await;
        let result = sqlx::query!(r#"SELECT VERSION()"#).fetch_one(&pool).await;

        tracing::info!("Health check: {:?}", result);
        Json(json!({ "message": "All is well", "code": 200 }))
    }
    route("/health", get(handler))
}
