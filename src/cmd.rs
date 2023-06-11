// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password};
use sqlx::{migrate::Migrator, query, Pool, Postgres};
use uuid::Uuid;

use crate::{utils, BUILD_TIME, PKG_ARCH, PKG_NAME, PKG_OS, PKG_VERSION};

#[derive(rust_embed::RustEmbed)]
#[folder = "migrations/"]
#[include = "*.sql"]
struct Migrations;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub fn version(short: bool) {
	if short {
		println!("{}", PKG_VERSION);
	} else {
		println!("{}", about());
	}
}

pub fn about() -> String {
	let platform = format!("{}-{}", PKG_ARCH, PKG_OS);
	format!("{} {} {} ({})", PKG_NAME, PKG_VERSION, platform, BUILD_TIME)
}

pub fn generate_secret() -> String {
	let mut bytes = [0u8; 40];
	getrandom::getrandom(&mut bytes)
		.unwrap_or_else(|err| panic!("could not retrieve random bytes: {}", err));

	let chars = crate::ALPHABET_62;
	let mask = chars.len() - 1;

	bytes
		.iter_mut()
		.for_each(|b| *b = chars[*b as usize & mask]);

	String::from_utf8_lossy(&bytes).to_string()
}

pub async fn migrate(force: bool) {
	let uri = utils::get_envar("DATABASE_URL", None);
	let pool = Pool::<Postgres>::connect(&uri).await.unwrap();

	let success_message = "🍀 Database migration succes";
	let failed_message = "🍀 Could not execute database migration";

	if force {
		println!("🍀 Running database migration automatically");
		match migrate_up(pool).await {
			Ok(_) => println!("{}", success_message),
			Err(e) => println!("{}: {:?}", failed_message, e),
		}
	} else if Confirm::with_theme(&ColorfulTheme::default())
		.with_prompt("Do you want to continue?")
		.wait_for_newline(true)
		.interact()
		.unwrap()
	{
		match migrate_up(pool).await {
			Ok(_) => println!("{}", success_message),
			Err(e) => println!("{}: {:?}", failed_message, e),
		}
	} else {
		println!("Sayonara 👋");
	}
}

async fn migrate_up(pool: Pool<Postgres>) -> Result<(), sqlx::Error> {
	for mf in MIGRATOR.iter() {
		println!("🚀 Running migration {:} - {:}", mf.version, mf.description);
		MIGRATOR.run(&pool).await.unwrap();
	}
	Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
struct Account {
	id: Uuid,
	email: String,
	password: String,
}

pub async fn create_admin() {
	println!("Welcome to the setup wizard");

	let email = Input::with_theme(&ColorfulTheme::default())
		.with_prompt("Email address")
		.validate_with({
			move |input: &String| -> Result<(), &str> {
				if utils::is_valid_email(input) {
					Ok(())
				} else {
					Err("This is not a mail address!")
				}
			}
		})
		.interact();

	let password = Password::with_theme(&ColorfulTheme::default())
		.with_prompt("Password")
		.with_confirmation("Repeat password", "Error: the passwords don't match.")
		.interact();

	if Confirm::with_theme(&ColorfulTheme::default())
		.with_prompt("Do you want to continue?")
		.wait_for_newline(true)
		.interact()
		.unwrap()
	{
		match process_create_admin(email.unwrap(), password.unwrap()).await {
			Ok(v) => println!("Admin with email {:?} has been created!", v.email),
			Err(e) => println!("Could not create admin: {:?}", e),
		}
	} else {
		println!("Sayonara 👋");
	}
}

async fn process_create_admin(email: String, password: String) -> Result<Account, sqlx::Error> {
	let uri = utils::get_envar("DATABASE_URL", None);
	let pool = Pool::<Postgres>::connect(&uri).await.unwrap();

	let user = Account {
		id: Uuid::new_v4(),
		email,
		password,
	};

	// TODO Check is email already exists.

	let sql = r#"
    WITH new_user AS (
      INSERT INTO users (id, uid, email, is_super_admin) VALUES ($1, $2, $3, $4) returning id
    )
    INSERT INTO passwords (user_id, encrypted_password) VALUES ((select id from new_user), $5)
  "#;

	let password_hash: String = password_auth::generate_hash(&user.password);

	query(sql)
		.bind(user.id)
		.bind(utils::generate_uid())
		.bind(user.email.to_string())
		.bind(true)
		.bind(password_hash)
		.execute(&pool)
		.await?;

	Ok(user)
}
