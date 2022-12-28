use axum::{http::StatusCode, response::Html, routing::get, Router};

use crate::{router::route, utils};

pub fn hello() -> Router {
    async fn handler() -> Html<&'static str> {
        Html("<p>All is well!</p>")
    }
    route("/api", get(handler))
}

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
