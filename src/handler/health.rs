use axum::{http::StatusCode, routing::get, Router};

use crate::{routes::route, utils};

#[utoipa::path(
    get,
    path = "/health",
    tag = "Information",
    responses(
        (status = 200, description = "Health check endpoint")
    )
)]
pub fn health_check() -> Router {
    async fn handler() -> Result<String, (StatusCode, String)> {
        let pool = crate::config::database::connection_pool().await;
        let query = sqlx::query_scalar("SELECT VERSION()")
            .fetch_one(&pool)
            .await
            .map_err(utils::error::internal_error);

        tracing::info!("Healt check: {:?}", query);
        return query;
    }
    route("/health", get(handler))
}
