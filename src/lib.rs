// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

extern crate cookie;

use salvo::logging::Logger;
use salvo::prelude::*;
use salvo::proxy::Proxy;
use salvo::serve_static::static_embed;

use std::{net::SocketAddr, time::Duration};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// use crate::state::AppState;
use crate::handler::{error::throw_response, root::health_check};

pub mod config;
pub mod handler;
pub mod middleware;
pub mod routes;
pub mod service;
pub mod state;
pub mod swagger;
pub mod utils;

#[derive(rust_embed::RustEmbed)]
#[folder = "web/"]
struct Assets;

pub async fn run() {
    let tracing_filter = "fastrue=debug,salvo=info";
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| tracing_filter.into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // let state = AppState {
    //     db: config::connection_pool().await,
    // };

    // Setup connection pool and register application router
    // Add a fallback service for handling routes to unknown paths

    let web_ui: Router = if cfg!(debug_assertions) {
        Router::with_path("<**rest>").handle(Proxy::new(vec!["http://localhost:3000"]))
    } else {
        Router::with_path("ui/<**path>").get(static_embed::<Assets>().fallback("index.html"))
    };

    let router = Router::new()
        .hoop(CatchPanic::new())
        .hoop(Logger::new())
        .get(health_check)
        .push(Router::with_path("api").get(health_check))
        .push(Router::with_path("health").get(health_check))
        .push(Router::with_path("error").get(throw_response))
        .push(web_ui);

    tokio::join!(serve(router));
}

// Start the server
async fn serve(router: Router) {
    let addr: SocketAddr = config::app::bind_addr()
        .parse()
        .expect("Unable to parse socket address");

    if cfg!(debug_assertions) {
        tracing::info!("👀 [DEV] Server listening on http://{}", addr);
    } else {
        tracing::info!("👀 Server listening on http://{}", addr);
    }

    let acceptor = TcpListener::new(addr).bind().await;
    Server::new(acceptor)
        .serve_with_graceful_shutdown(router, shutdown_signal(), Some(Duration::from_secs(10)))
        .await;
}

// Graceful shutdown
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
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
