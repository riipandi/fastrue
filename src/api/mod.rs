use axum::{routing::MethodRouter, Router};

pub mod error_handler;
pub mod root;
pub mod send_email;

pub fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}
