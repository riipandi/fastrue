extern crate cookie;

use axum::http::Request;
use axum::response::Response;
use axum::Router;
use std::{net::SocketAddr, time::Duration};
use tokio::signal;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::state::AppState;

pub mod config;
pub mod handler;
pub mod middleware;
pub mod routes;
pub mod service;
pub mod state;
pub mod swagger;
pub mod utils;

pub async fn run() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "fastrue=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState {
        db: config::connection_pool().await,
    };

    // Setup connection pool and register application router
    // Add a fallback service for handling routes to unknown paths

    let app = routes::register_routes(state);

    tokio::join!(serve(app));
}

// Start the server
async fn serve(app: Router) {
    // Configure logging middleware
    let trace_layer = TraceLayer::new_for_http()
        .on_request(|request: &Request<_>, _span: &Span| {
            tracing::info!("Request {} {}", request.method(), request.uri());
        })
        .on_response(|response: &Response, latency: Duration, _span: &Span| {
            tracing::info!("Response {} {:?}", response.status(), latency);
        })
        .on_failure(
            |error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                tracing::error!("Failure {} {:?}", error, latency);
            },
        );

    let addr: SocketAddr = config::app::bind_addr()
        .parse()
        .expect("Unable to parse socket address");

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.layer(trace_layer).into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
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
