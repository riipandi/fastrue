use axum::{
    response::{IntoResponse, Json},
    routing::post,
    Router,
};
use serde_json::json;

use crate::routes::route;

#[utoipa::path(
    post,
    path = "/api/token",
    tag = "Authentication",
    responses(
        (status = 200, description = "Retrieve a token")
    ),
)]
pub fn post_token() -> Router {
    async fn handler() -> impl IntoResponse {
        Json(json!({
          "message": "Not yet implemented"
        }))
    }
    route("/token", post(handler))
}
