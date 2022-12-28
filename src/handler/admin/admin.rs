use axum::{
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde_json::json;

use crate::routes::route;

pub fn get_all_admin() -> Router {
    async fn handler() -> impl IntoResponse {
        Json(json!({
          "message": "Not yet implemented"
        }))
    }
    route("/admin", get(handler))
}
