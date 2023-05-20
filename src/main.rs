// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use fastrue::config::get_envar;
use fastrue::service::create_admin;
use fastrue::utils::{migration::run_migration, string::generate_secret};

#[derive(Parser)]
#[command(author, about, long_about = None)]
#[command(version, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/*
 * More example here: https://github.com/dirien/rust-cli/blob/main/src/main.rs
 */
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
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables

    // You can check for the existence of subcommands, and if found
    // use their matches just as you would the top level command.
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::GenerateSecret {}) => println!("{}", generate_secret()),
        Some(Commands::Migrate { force }) => run_migration(force).await,
        Some(Commands::CreateAdmin {}) => create_admin::prompt().await,
        None => {
            let auto_migrate = get_envar("FASTRUE_AUTO_MIGRATE", Some("true"));
            if auto_migrate.trim().parse().unwrap() {
                println!("🍀 Running automatic database migration");
                run_migration(true).await
            }
            fastrue::run().await;
        }
    }
}
