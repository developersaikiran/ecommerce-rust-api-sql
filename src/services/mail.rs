use std::env;

use lettre::message::{header::ContentType, Mailbox, Message};
use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;
use lettre::Transport;
use dotenv::dotenv;


pub async fn send_registration_email(to: &str, subject: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();
    let EMAIL_USERNAME = env::var(format!("EMAIL_USERNAME_{}", env::var("RUN_MODE").unwrap()) ).unwrap();
    let EMAIL_PASSWORD = env::var(format!("EMAIL_PASSWORD_{}", env::var("RUN_MODE").unwrap()) ).unwrap();
    
    let email = Message::builder()
        .from(EMAIL_USERNAME.parse::<Mailbox>()?)
        .reply_to(EMAIL_USERNAME.parse::<Mailbox>()?)
        .to(to.parse::<Mailbox>()?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())?;

    // Replace with your SMTP server credentials
    let creds = Credentials::new(EMAIL_USERNAME.to_string(), EMAIL_PASSWORD.to_string());

    let mailer = SmtpTransport::starttls_relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    Ok(())
}