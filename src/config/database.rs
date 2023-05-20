use sqlx::{postgres::PgPoolOptions, PgPool};

use super::get_envar;

// pub async fn connection_pool() -> Result<sqlx::Pool<PgConnection, SqliteConnection>, Box<dyn std::error::Error>> {
pub async fn connection_pool() -> PgPool {
    // let connection_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let default_conn_str = "postgres://postgres:postgres@127.0.0.1:5432/fastrue";
    let connection_str = get_envar("DATABASE_URL", Some(default_conn_str));
    tracing::debug!("Using database connection {}", connection_str);

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_str)
        .await
        .expect("Can't connect to database")
}
