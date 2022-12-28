use clap::{Parser, Subcommand};
use rand::{distributions::Alphanumeric, Rng};

mod api;
mod config;
mod middleware;
mod utils;

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
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found
    // use their matches just as you would the top level cmd.
    match &cli.command {
        Some(Commands::GenerateSecret {}) => {
            let s: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(40)
                .map(char::from)
                .collect();
            println!("{}", s);
        }
        None => api::serve::serve().await,
    }
}
