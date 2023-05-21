// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

pub mod config;
pub mod entities;
pub mod handler;
pub mod middleware;
pub mod routes;
pub mod service;
pub mod state;
pub mod swagger;
pub mod utils;

extern crate cookie;

use salvo::prelude::*;
use std::{net::SocketAddr, time::Duration};

// Start the server
pub async fn serve() {
    let addr: SocketAddr = config::bind_addr()
        .parse()
        .expect("Unable to parse socket address");

    if cfg!(debug_assertions) {
        tracing::info!("👀 [DEV] Server listening on http://{}", addr);
    } else {
        tracing::info!("👀 Server listening on http://{}", addr);
    }

    let acceptor = TcpListener::new(addr).bind().await;
    let graceful_timeout = if cfg!(debug_assertions) { 0 } else { 10 };

    Server::new(acceptor)
        .serve_with_graceful_shutdown(
            routes::create_service(),
            shutdown_signal(),
            Some(Duration::from_secs(graceful_timeout)),
        )
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

    tracing::info!("signal received, starting graceful shutdown");
}
