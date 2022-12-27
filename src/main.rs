use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use sqlx::postgres::{PgPool, PgPoolOptions};
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

    // Read database connection from envars
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432".to_string());

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route(
            "/dbconn",
            get(using_connection_pool_extractor).post(using_connection_extractor),
        )
        .with_state(pool)
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

// ---------------------------------------------------------------------------------------------------------

// we can extract the connection pool with `State`
async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
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
