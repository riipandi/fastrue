use axum::{
    response::{IntoResponse, Json},
    routing::{get, put},
    Router,
};
use serde_json::json;

use crate::routes::route;

pub fn get_user() -> Router {
    async fn handler() -> impl IntoResponse {
        Json(json!({
          "message": "Not yet implemented"
        }))
    }
    route("/user", get(handler))
}

pub fn put_user() -> Router {
    async fn handler() -> impl IntoResponse {
        Json(json!({
          "message": "Not yet implemented"
        }))
    }
    route("/user", put(handler))
}
