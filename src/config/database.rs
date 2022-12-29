use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

use super::set_default_envar;

// pub(crate) async fn connection_pool() -> Result<sqlx::Pool<PgConnection, SqliteConnection>, Box<dyn std::error::Error>> {
pub(crate) async fn connection_pool() -> PgPool {
    // let connection_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let default_conn_str = "postgres://postgres:postgres@127.0.0.1:5432/wasta";
    set_default_envar("DATABASE_URL", default_conn_str);
    let connection_str = env::var("DATABASE_URL").unwrap();
    tracing::debug!("Using database connection {}", connection_str);

    return PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_str)
        .await
        .expect("Can't connect to database");
}
