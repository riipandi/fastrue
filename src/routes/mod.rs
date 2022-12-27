use axum::{routing::MethodRouter, Router};

pub fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}

pub mod mailer;
pub mod root;
