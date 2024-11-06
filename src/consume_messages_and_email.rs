use anyhow::Result;
use futures::TryStreamExt;
use iggy::clients::client::IggyClient;
use lettre::{
    message::header::ContentType,
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
    Message, SmtpTransport, Transport,
};
use serde::{Deserialize, Serialize};
use std::env;
use tracing::info;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePayload {
    timestamp: String,
    action: String,
    parameters: Vec<String>,
}

pub struct EmailConfig {
    pub from_email: String,
    pub to_email: String,
    pub subject: String,
    pub body: String,
}

use dotenv::dotenv;
pub async fn consume_messages_and_email(
    client: &IggyClient,
    tenant: &str,
    topic: &str,
    email_config: EmailConfig,
) -> Result<()> {
    dotenv().ok();
    let mut consumer = client
        .consumer_group("display_group", tenant, topic)?
        .create_consumer_group_if_not_exists()
        .auto_join_consumer_group()
        .build();

    consumer.init().await?;
    info!(
        "Started consuming messages from tenant: {}, topic: {}",
        tenant, topic
    );

    let from_email = env::var("EMAIL_USER").unwrap_or_else(|_| {
        eprintln!("EMAIL_USER not found");
        std::process::exit(1);
    });

    // Setup email configuration
    let password = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD must be set");
    let creds = Credentials::new(from_email.clone(), password);
    let tls_parameters = TlsParameters::new("smtp.gmail.com".to_string())?;

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .tls(Tls::Opportunistic(tls_parameters))
        .port(587)
        .build();

    while let Ok(Some(message)) = consumer.try_next().await {
        match String::from_utf8(message.message.payload.to_vec()) {
            Ok(json_str) => match serde_json::from_str::<MessagePayload>(&json_str) {
                Ok(payload) => {
                    // Construct email body
                    let email_body = format!(
                        "=== Message Received ===\n\
                        Tenant (Stream): {}\n\
                        Topic: {}\n\
                        Action: {}\n\
                        Parameters:\n{}\n\
                        Timestamp: {}\n\
                        =====================",
                        tenant,
                        topic,
                        payload.action,
                        payload
                            .parameters
                            .iter()
                            .enumerate()
                            .map(|(i, param)| format!(" {}: {}", i + 1, param))
                            .collect::<Vec<_>>()
                            .join("\n"),
                        payload.timestamp
                    );

                    // Create the email
                    let email = Message::builder()
                        .from(email_config.from_email.parse()?)
                        .to(email_config.to_email.parse()?)
                        .subject(&email_config.subject)
                        .header(ContentType::TEXT_PLAIN)
                        .body(email_body)?;

                    // Send the email
                    match mailer.send(&email) {
                        Ok(_) => info!("Email notification sent successfully"),
                        Err(e) => eprintln!("Failed to send email: {}", e),
                    }
                }
                Err(e) => eprintln!("Error parsing message JSON: {}", e),
            },
            Err(e) => eprintln!("Error reading message payload: {}", e),
        }
    }

    Ok(())
}
