use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password};
use sqlx::query;
use uuid::Uuid;

use crate::{
    config,
    utils::{create_hash, is_valid_email},
};

#[derive(Debug)]
#[allow(dead_code)]
struct Account {
    email: String,
    password: String,
}

pub async fn prompt() {
    println!("Welcome to the setup wizard");

    let email = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Email address")
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                if is_valid_email(input) {
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
        match create_admin(email.unwrap(), password.unwrap()).await {
            Ok(v) => println!("Admin with email {:?} has been created!", v.email),
            Err(e) => println!("Could not create admin: {:?}", e),
        }
    } else {
        println!("Sayonara 👋");
    }
}

async fn create_admin(email: String, password: String) -> Result<Account, sqlx::Error> {
    let id = Uuid::new_v4();
    let password_hash: String = create_hash(password.clone());

    // FIXME: Fix database connection when build inside Docker
    let pool = config::connection_pool().await;

    query(
        r#"INSERT INTO users (id, email, encrypted_password, is_super_admin)
        VALUES ($1, $2, $3, $4)"#,
    )
    .bind(id)
    .bind(email.to_string())
    .bind(password_hash.to_string())
    .bind(true)
    .execute(&pool)
    .await?;

    // Check that inserted todo can be fetched inside the uncommitted transaction
    let _ = query!(r#"SELECT FROM users WHERE id = $1"#, id)
        .fetch_one(&pool)
        .await?;

    // Result to pass
    let data = Account { email, password };

    Ok(data)
}
