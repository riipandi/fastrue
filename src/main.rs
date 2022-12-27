use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, HeaderMap, Method, Request, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use axum_extra::routing::SpaRouter;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{env, io, net::SocketAddr, time::Duration};
use tokio::signal;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// modules
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "wasta=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Read database connection from envars
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432".to_string());

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("Can't connect to database");
    println!("Successfully connected to database");

    // Static SPA assets
    let spa = SpaRouter::new("/spa", "web").handle_error(handle_spa_error);

    let app = Router::new()
        .merge(spa)
        .route(
            "/health",
            get(using_connection_pool_extractor).post(using_connection_extractor),
        )
        .with_state(pool)
        .merge(routes::root::root())
        .merge(routes::root::get_foo())
        .merge(routes::root::post_foo())
        .merge(routes::mailer::get_send_email())
        .layer(TraceLayer::new_for_http())
        .layer(
            TraceLayer::new_for_http()
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // println!("message log on_request");
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    // println!("message log on_response");
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    // println!("message log on_body_chunk");
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // println!("message log on_eos");
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        // println!("message log on_failure");
                    },
                ),
        );

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(handler_404);

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

async fn handle_spa_error(method: Method, uri: Uri, err: io::Error) -> String {
    format!("{} {} failed with {}", method, uri, err)
}

// ---------------------------------------------------------------------------------------------------------

// we can extract the connection pool with `State`
async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("SELECT version()")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

async fn using_connection_extractor(
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    let mut conn = conn;
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&mut conn)
        .await
        .map_err(internal_error)
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
