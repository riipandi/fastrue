use axum::{http::StatusCode, response::IntoResponse, Router};
use std::{env, net::SocketAddr};
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// modules
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_global_404_handler=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .merge(routes::root())
        .merge(routes::get_foo())
        .merge(routes::post_foo());

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(handler_404);

    // Read bind address from envar or set the default.
    set_default_envar("BIND_PORT", "3030");
    set_default_envar("BIND_ADDR", "127.0.0.1");
    let env_port = env::var("BIND_PORT").unwrap();
    let env_addr = env::var("BIND_ADDR").unwrap();
    let bind_addr = [env_addr, env_port].join(":");

    let addr: SocketAddr = bind_addr.parse().expect("Unable to parse socket address");
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

// Set environment variable value
fn set_default_envar(key: &str, value: &str) {
    if env::var(key).is_err() {
        env::set_var(key, value);
    }
}

// Global 404 handler
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

// Graceful shutdown
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
