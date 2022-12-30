use axum::{
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde_json::json;

use crate::routes::route;

#[utoipa::path(
    get,
    path = "/admin",
    tag = "Administration",
    responses(
        (status = 200, description = "Get all admin users")
    ),
)]
pub fn get_all_admin() -> Router {
    async fn handler() -> impl IntoResponse {
        Json(json!({
          "message": "Not yet implemented"
        }))
    }
    route("/admin", get(handler))
}
