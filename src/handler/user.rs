use axum::{
    response::{IntoResponse, Json},
    routing::{get, put},
    Router,
};
use serde_json::json;

use crate::routes::route;

#[utoipa::path(
    get,
    path = "/user",
    tag = "User Account",
    responses(
        (status = 200, description = "Get all users")
    ),
)]
pub fn get_user() -> Router {
    async fn handler() -> impl IntoResponse {
        Json(json!({
          "message": "Not yet implemented"
        }))
    }
    route("/user", get(handler))
}

#[utoipa::path(
    put,
    path = "/user",
    tag = "User Account",
    responses(
        (status = 200, description = "Update a user")
    ),
)]
pub fn put_user() -> Router {
    async fn handler() -> impl IntoResponse {
        // let pool = config::connection_pool().await;
        // query(r#"select * from users"#).execute(&pool).await?;

        Json(json!({
          "message": "Not yet implemented"
        }))
    }
    route("/user", put(handler))
}
