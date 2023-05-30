// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use fastrue::service::create_admin;
use fastrue::utils::{migration::run_migration, string::generate_secret};
use fastrue::{config, state};

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/**
 * Clap references:
 * - https://github.com/dirien/rust-cli
 * - https://github.com/dirien/rust-cli/blob/main/src/main.rs
 * - https://github.com/clap-rs/clap/discussions/4720#discussioncomment-5066184
 **/
#[derive(Subcommand)]
enum Commands {
    /// Create administrator user
    CreateAdmin {},
    /// Generate application secret key
    GenerateSecret {},
    /// Run the database migration
    Migrate {
        /// Force run, disable confirmation prompt
        #[arg(short = 'f', long = "force", default_value_t = false)]
        force: bool,
    },
    /// Print application version
    Version {
        /// Print short version number
        #[arg(short = 's', long = "short", default_value_t = false)]
        short: bool,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables

    let tracing_filter = "fastrue=debug,salvo=info,sqlx=error";
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| tracing_filter.into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // You can check for the existence of subcommands, and if found
    // use their matches just as you would the top level command.
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::GenerateSecret {}) => println!("{}", generate_secret()),
        Some(Commands::Version { short }) => {
            if short {
                println!("{}", PKG_VERSION);
            } else {
                let build_timestamp = build_time::build_time_local!("%Y-%m-%d %:z");
                println!("{} {} ({})", PKG_NAME, PKG_VERSION, build_timestamp);
            }
        }
        Some(Commands::Migrate { force }) => {
            // Open database connection
            if let Err(err) = futures::executor::block_on(open_db()) {
                panic!("Cannot connect to database: {}", err);
            }
            run_migration(force).await
        }
        Some(Commands::CreateAdmin {}) => create_admin::prompt().await,
        None => {
            let build_timestamp = build_time::build_time_utc!();
            println!("\n{} {} ({}).\n", PKG_NAME, PKG_VERSION, build_timestamp);

            // Open database connection
            if let Err(err) = futures::executor::block_on(open_db()) {
                panic!("Cannot connect to database: {}", err);
            }

            let auto_migrate = config::get_envar("FASTRUE_AUTO_MIGRATE", Some("true"));
            if auto_migrate.trim().parse().unwrap() {
                tracing::info!("🍀 Running automatic database migration");
                run_migration(true).await
            }

            tokio::join!(fastrue::serve());
        }
    }
}

async fn open_db() -> Result<(), sqlx::Error> {
    let default_conn_str = "postgres://postgres:postgres@127.0.0.1:5432/fastrue";
    let connection_str = config::get_envar("DATABASE_URL", Some(default_conn_str));

    tracing::info!("👀 Using database connection {}", connection_str);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10) // Set the maximum number of connections in the pool
        .min_connections(2) // Set the minimum number of connections to keep in the pool
        .acquire_timeout(std::time::Duration::from_secs(5)) // Set the connection timeout duration
        .connect(&connection_str)
        .await
        .unwrap();

    state::POSTGRES.set(pool).unwrap();
    sqlx::query("SELECT 1").fetch_one(state::dbconn()).await?;

    Ok(())
}
