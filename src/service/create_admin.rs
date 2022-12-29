use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password};

use crate::utils::{hash::create_hash, validator::is_valid_email};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    email: String,
    password: String,
}

pub fn prompt() {
    println!("Welcome to the setup wizard");

    let email = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Email address")
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                if is_valid_email(&input) {
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
        match create_admin(email.unwrap(), password.unwrap()) {
            Ok(_) => tracing::info!("Admin has been created"),
            Err(e) => tracing::error!("Could not create admin: {:?}", e),
        }
    } else {
        println!("Sayonara 👋");
    }
}

fn create_admin(email: String, password: String) -> Result<(), Box<dyn std::error::Error>> {
    let password_hash: String = create_hash(password);
    println!("{:?} {:?}", email, password_hash);

    Ok(())
}
