extern crate lettre;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use std::env;

use crate::config::set_default_envar;

// Email sending function
pub async fn send_email_smtp(
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read smtp configuration from envar or set the default.
    set_default_envar("SMTP_HOST", "localhost");
    set_default_envar("SMTP_PORT", "1025");
    set_default_envar("SMTP_USERNAME", "");
    set_default_envar("SMTP_PASSWORD", "");
    set_default_envar("SMTP_SECURE", "false");

    let env_smtp_port = env::var("SMTP_PORT").unwrap();
    let smtp_host = env::var("SMTP_HOST").unwrap();
    let smtp_username = env::var("SMTP_USERNAME").unwrap();
    let smtp_password = env::var("SMTP_PASSWORD").unwrap();
    let smtp_secure = env::var("SMTP_SECURE").unwrap();
    let smtp_port: u16 = env_smtp_port.parse().unwrap();

    // Build email message
    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    // Send the email using SMTP transport
    if smtp_secure.trim().parse().unwrap() {
        let smtp_credentials = Credentials::new(smtp_username, smtp_password);
        AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)
            .unwrap()
            .port(smtp_port)
            .credentials(smtp_credentials)
            .build()
            .send(email)
            .await?;
    } else {
        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(smtp_host)
            .port(smtp_port)
            .build()
            .send(email)
            .await?;
    }

    Ok(())
}
