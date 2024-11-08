mod config;
mod messaging;
mod email;

use anyhow::Result;
use iggy::client::{Client, UserClient};
use iggy::clients::builder::IggyClientBuilder;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};
use config::{smtp::SmtpConfig, email::EmailConfig};
use email::sender::EmailSender;
use messaging::consumer::MessageConsumer;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging
    Registry::default()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("INFO")))
        .init();

    // Connect and authenticate
    let client = IggyClientBuilder::new()
        .with_tcp()
        .with_server_address("abjad.mayorana.ch:8090".to_string())
        .build()?;
    client.connect().await?;
    client.login_user("iggy", "iggy").await?;

    // Initialize configurations
    let smtp_config = SmtpConfig::from_env()?;
    let email_config = EmailConfig::from_env();
    let email_sender = EmailSender::new(&smtp_config, email_config)?;

    // Setup consumer with client and email sender
    let consumer = MessageConsumer::new(client, email_sender);

    // Start consuming messages
    consumer.consume_messages("gibro", "notification").await?;

    Ok(())
}
