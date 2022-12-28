use crate::utils::set_default_envar;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub fn connection_url() -> String {
    let default_url = "postgres://postgres:postgres@127.0.0.1:5432";
    set_default_envar("DATABASE_URL", default_url);
    let connection_string = env::var("DATABASE_URL").unwrap();
    connection_string
}

pub async fn connection_pool() -> PgPool {
    let connection_string = connection_url();
    return PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await
        .expect("Can't connect to database");
}
