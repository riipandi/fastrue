use axum::{
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde_json::json;

use crate::routes::route;

#[utoipa::path(
    get,
    path = "/api/signup",
    tag = "Authentication",
    responses(
        (status = 200, description = "Register an account")
    ),
)]
pub fn signup() -> Router {
    async fn handler() -> impl IntoResponse {
        Json(json!({
          "message": "Not yet implemented"
        }))
    }
    route("/signup", get(handler))
}
