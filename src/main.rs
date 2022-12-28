use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use wasta::utils::{migration::run_migration, string::generate_secret};

#[derive(Parser)]
#[command(author, about, long_about = None)]
#[command(version, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate application secret key
    GenerateSecret {},
    /// Run the database migration
    Migrate {},
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load dotenv
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found
    // use their matches just as you would the top level cmd.
    match &cli.command {
        Some(Commands::GenerateSecret {}) => {
            println!("{}", generate_secret());
        }
        Some(Commands::Migrate {}) => {
            run_migration().await;
        }
        None => wasta::run().await,
    }
}
