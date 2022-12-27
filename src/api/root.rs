use axum::{
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};

use super::route;

pub fn root() -> Router {
    route(
        "/",
        get(|| async { String::from("Hello, world!").into_response() }),
    )
}

pub fn get_foo() -> Router {
    async fn handler() -> Json<Value> {
        Json(json!({
          "message": "Hello from foo endpoint",
          "code": 200,
        }))
    }

    route("/api/foo", get(handler))
}

pub fn post_foo() -> Router {
    async fn handler() -> &'static str {
        "Hi from `POST /api/foo`"
    }

    route("/api/foo", post(handler))
}
