extern crate lettre;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use crate::config::get_envar;

// Email sending function
pub async fn send_email_smtp(
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read smtp configuration from envar or set the default.
    let smtp_host = get_envar("TRUSTY_SMTP_HOST", Some("localhost"));
    let env_smtp_port = get_envar("TRUSTY_SMTP_PORT", Some("1025"));
    let smtp_username = get_envar("TRUSTY_SMTP_USERNAME", None);
    let smtp_password = get_envar("TRUSTY_SMTP_PASSWORD", None);
    let smtp_secure = get_envar("TRUSTY_SMTP_SECURE", None);
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
