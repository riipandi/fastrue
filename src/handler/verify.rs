use axum::{
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde_json::json;

use crate::routes::route;

pub fn verify() -> Router {
    async fn handler() -> impl IntoResponse {
        Json(json!({
          "message": "Not yet implemented"
        }))
    }
    route("/verify", get(handler))
}
