use anyhow::Result;
use futures::TryStreamExt;
use iggy::clients::client::IggyClient;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePayload {
    timestamp: String,
    action: String,
    parameters: Vec<String>,
}

pub async fn consume_messages(client: &IggyClient, tenant: &str, topic: &str) -> Result<()> {
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

    while let Ok(Some(message)) = consumer.try_next().await {
        match String::from_utf8(message.message.payload.to_vec()) {
            Ok(json_str) => match serde_json::from_str::<MessagePayload>(&json_str) {
                Ok(payload) => {
                    println!("\n=== Message Received ===");
                    println!("Tenant (Stream): {}", tenant);
                    println!("Topic: {}", topic);
                    println!("Action: {}", payload.action);
                    println!("Parameters:");
                    for (index, param) in payload.parameters.iter().enumerate() {
                        println!("  {}: {}", index + 1, param);
                    }
                    println!("Timestamp: {}", payload.timestamp);
                    println!("=====================\n");
                }
                Err(e) => println!("Error parsing message JSON: {}", e),
            },
            Err(e) => println!("Error reading message payload: {}", e),
        }
    }

    Ok(())
}
