use axum::{
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde_json::json;

use crate::{config, routes::route, service::user::User};

#[utoipa::path(
    get,
    path = "/api/admin",
    tag = "Administration",
    responses(
        (status = 200, description = "Get all admin users")
    ),
)]
pub fn get_all_users() -> Router {
    async fn handler() -> impl IntoResponse {
        let pool = config::connection_pool().await;
        let query = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&pool)
            .await;

        match query {
            Ok(users) => Json(json!(users)),
            Err(_) => Json(json!({
              "message": "No user found"
            })),
        }
    }
    route("/admin", get(handler))
}
