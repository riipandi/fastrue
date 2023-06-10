// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use clap::{Parser, Subcommand};
use fastrue::{cmd, server};

#[derive(Parser)]
#[command(about, long_about = None)]
struct Args {
	/// Address to bind
	#[arg(short = 'a', long = "address", default_value = "0.0.0.0")]
	address: String,
	/// Port to listen
	#[arg(short = 'p', long = "port", default_value = "9090")]
	port: String,
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	/// Generate application secret key
	GenerateSecret {},
	/// Create administrator user
	CreateAdmin {},
	/// Run database migration
	Migrate {
		/// Force run, disable confirmation prompt
		#[arg(short = 'f', long = "force", default_value_t = false)]
		force: bool,
	},
	/// Print version information
	Version {
		/// Print short version number
		#[arg(short = 's', long = "short", default_value_t = false)]
		short: bool,
	},
}

#[tokio::main]
async fn main() {
	dotenvy::dotenv().ok(); // Load environment variables

	// You can check for the existence of subcommands, and if found
	// use their matches just as you would the top level command.
	let args = Args::parse();

	match args.command {
		Some(Commands::GenerateSecret {}) => println!("{}", cmd::generate_secret()),
		Some(Commands::Migrate { force }) => cmd::migrate(force).await,
		Some(Commands::CreateAdmin {}) => cmd::create_admin().await,
		Some(Commands::Version { short }) => cmd::version(short),
		None => {
			// Do whatever you want here, finally start the server.
			let addr = [args.address, args.port.to_string()].join(":");
			server::run(addr).await;
		}
	}
}
