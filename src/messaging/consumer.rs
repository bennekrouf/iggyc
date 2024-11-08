use anyhow::Result;
use futures::TryStreamExt;
use iggy::clients::client::IggyClient;
use tracing::info;
use crate::{
    messaging::payload::MessagePayload,
    email::sender::EmailSender,
};

pub struct MessageConsumer {
    client: IggyClient,
    email_sender: EmailSender,
}

impl MessageConsumer {
    pub fn new(client: IggyClient, email_sender: EmailSender) -> Self {
        Self {
            client,
            email_sender,
        }
    }

    pub async fn consume_messages(&self, tenant: &str, topic: &str) -> Result<()> {
        let mut consumer = self.client
            .consumer_group("display_group", tenant, topic)?
            .create_consumer_group_if_not_exists()
            .auto_join_consumer_group()
            .build();

        consumer.init().await?;
        info!(
            "Started consuming messages from tenant: {}, topic: {}",
            tenant, topic
        );

        while let Ok(Some(message)) = consumer.try_next().await {
            if let Ok(json_str) = String::from_utf8(message.message.payload.to_vec()) {
                if let Ok(payload) = serde_json::from_str::<MessagePayload>(&json_str) {
                    // Get email address from parameters
                    if let Some(to_email) = payload.get_parameter_value("email") {
                        let email_body = format!(
                            "Action: {}\nDescription: {}\n\nParameters:\n{}",
                            payload.text,
                            payload.description,
                            payload.parameters
                                .iter()
                                .map(|p| format!("{}: {}", p.name, p.value))
                                .collect::<Vec<_>>()
                                .join("\n")
                        );

                        if let Err(e) = self.email_sender.send_to(&to_email, &payload.text, email_body) {
                            eprintln!("Failed to send email: {}", e);
                        }
                    } else {
                        info!("No email parameter found in message");
                    }
                }
            }
        }
        Ok(())
    }
}
