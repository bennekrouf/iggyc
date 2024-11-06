mod consume_messages;
mod consume_messages_and_email;
use anyhow::Result;
//use consume_messages::consume_messages;
use consume_messages_and_email::{consume_messages_and_email, EmailConfig};

use iggy::client::{Client, UserClient};
use iggy::clients::builder::IggyClientBuilder;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

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

    let tenant = "gibro"; // Change this to match your tenant/stream
    let topic = "notification";

    //consume_messages(&client, tenant, topic).await?;
    let email_config = EmailConfig {
        from_email: "mb@mayorana.ch".to_string(),
        to_email: "mohamed.bennekrouf@gmail.com".to_string(),
        subject: "New Message Notification".to_string(),
        body: "".to_string(), // Body is constructed from the message
    };

    consume_messages_and_email(&client, tenant, topic, email_config).await?;
    Ok(())
}
