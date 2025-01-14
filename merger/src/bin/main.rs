use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tanit::{
    application::{http::{HttpServer, HttpServerConfig}, ports::{MessagingPort, SubscriptionOptions}},
    infrastructure::messaging::kafka::Kafka,
};

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone)]
struct Ferry {
    id: String,
    #[allow(unused)]
    capacity: usize,
}

#[derive(Debug, Deserialize, Clone)]
struct CreateFerryEvent {
    id: String,
    capacity: usize,
}

pub async fn start_subscriptions(messaging: Arc<Kafka>) -> Result<()> {
    let messaging = Arc::clone(&messaging);
    let options = SubscriptionOptions {
        offset: tanit::application::ports::Offset::Beginning,
    };

    messaging
        .subscribe("ferris", "merger", options, {
            move |e: CreateFerryEvent| {
                println!("Received ferry: {:?}", e);

                async move { Ok(()) }
            }
        })
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let ferris: Arc<Mutex<HashMap<String, Ferry>>> = Arc::new(Mutex::new(HashMap::new()));

    let kafka = Arc::new(Kafka::new(
        "localhost:9092".to_string(),
        "example_group".to_string(),
    )?);

    start_subscriptions(Arc::clone(&kafka)).await?;

    let server_config = HttpServerConfig::new("3333".to_string());

    let http_server = HttpServer::new(server_config).await?;

    http_server.run().await?;

    Ok(())
}
