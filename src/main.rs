/*
 * Copyright 2022 Aris Ripandi <aris@duck.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use wasta::service::create_admin;
use wasta::utils::{migration::run_migration, string::generate_secret};

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
    better_panic::Settings::debug()
        .most_recent_first(false)
        .lineno_suffix(true)
        .install();
    dotenv().ok(); // Load environment variables

    // You can check for the existence of subcommands, and if found
    // use their matches just as you would the top level cmd.
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::GenerateSecret {}) => println!("{}", generate_secret()),
        Some(Commands::Migrate { force }) => run_migration(force).await,
        Some(Commands::CreateAdmin {}) => create_admin::prompt().await,
        None => wasta::run().await,
    }
}
