use futures::StreamExt;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    ClientConfig, Message, Offset, TopicPartitionList,
};
use serde::Deserialize;

#[derive(Debug, Clone)]
struct Ferry {
    id: String,
    #[allow(unused)]
    capacity: usize,
}

#[derive(Debug, Deserialize)]
struct CreateFerryEvent {
    id: String,
    capacity: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let ferris: Arc<Mutex<HashMap<String, Ferry>>> = Arc::new(Mutex::new(HashMap::new()));

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "example_group")
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .create()?;

    consumer.subscribe(&["test"])?;

    // reset the offset to the beginning
    let mut hash_map = HashMap::new();
    hash_map.insert(("test".to_string(), 0), Offset::Beginning);
    let t = TopicPartitionList::from_topic_map(&hash_map).unwrap();

    consumer.assign(&t)?;

    let ferries_clone = Arc::clone(&ferris);

    tokio::spawn(async move {
        println!("Starting consumer loop");
        let mut message_stream = consumer.stream();

        while let Some(result) = message_stream.next().await {
            match result {
                Ok(message) => {
                    if let Some(payload) = message.payload_view::<str>() {
                        match payload {
                            Ok(text) => {
                                println!("Received message: {}", text);
                                let event: CreateFerryEvent = serde_json::from_str(text).unwrap();

                                let ferry = Ferry {
                                    id: event.id,
                                    capacity: event.capacity,
                                };

                                let mut ferris = ferries_clone.lock().unwrap();
                                ferris.insert(ferry.id.clone(), ferry);
                            }
                            Err(e) => eprintln!("Error while reading message: {}", e),
                        }
                    }
                }
                Err(e) => eprintln!("Kafka error: {}", e),
            }
        }
    });

    tokio::signal::ctrl_c().await?;

    let ferris = ferris.lock().unwrap();
    println!("Ferries: {:?}", ferris);

    Ok(())
}
