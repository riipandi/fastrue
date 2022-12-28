use axum::{
    response::{IntoResponse, Json},
    routing::post,
    Router,
};
use serde_json::json;

use crate::routes::route;

pub fn post_logout() -> Router {
    async fn handler() -> impl IntoResponse {
        Json(json!({
          "message": "Not yet implemented"
        }))
    }
    route("/logout", post(handler))
}
