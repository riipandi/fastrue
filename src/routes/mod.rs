use axum::{
    response::{Html, Json},
    routing::{get, post, MethodRouter},
    Router,
};
use serde_json::{json, Value};

pub fn root() -> Router {
    async fn handler() -> Html<&'static str> {
        Html("<h1>Hello, World!</h1>")
    }

    route("/", get(handler))
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

fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}
