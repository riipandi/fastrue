use axum::{
    response::{IntoResponse, Json},
    routing::post,
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
    route("/verify", post(handler))
}
