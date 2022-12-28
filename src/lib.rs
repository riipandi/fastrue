use axum::{
    http::Request,
    response::{Redirect, Response},
    routing::get,
    Router,
};
use axum_extra::routing::SpaRouter;
use std::{env, net::SocketAddr, path::PathBuf, time::Duration};
use tokio::signal;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod middleware;
pub mod utils;

pub async fn run() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "wasta=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // This should be call create_router procedure

    // Static SPA assets (embedded)
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("web");
    let spa =
        SpaRouter::new("/spa", assets_dir).handle_error(api::error_handler::handler_spa_error);

    // setup connection pool
    let pool = config::database::connection_pool().await;

    let app = Router::new()
        .route("/", get(|| async { Redirect::temporary("/spa") }))
        .with_state(pool)
        .merge(api::root::hello())
        .merge(api::root::health_check())
        .merge(api::send_email::get_send_email())
        .merge(spa)
        .layer(
            TraceLayer::new_for_http()
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
                ),
        );

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(api::error_handler::handler_404);

    // Read bind address from envar or set the default.
    utils::set_default_envar("BIND_PORT", "3030");
    utils::set_default_envar("BIND_ADDR", "127.0.0.1");
    let env_port = env::var("BIND_PORT").unwrap();
    let env_addr = env::var("BIND_ADDR").unwrap();
    let bind_addr = [env_addr, env_port].join(":");

    let addr: SocketAddr = bind_addr.parse().expect("Unable to parse socket address");
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
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
