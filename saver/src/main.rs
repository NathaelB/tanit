use futures::StreamExt;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    ClientConfig, Message,
};

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "test-group")
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .unwrap();

    consumer.subscribe(&["test"]).unwrap();

    tokio::spawn(async move {
        println!("Consumer loop started");
        while let Some(result) = consumer.stream().next().await {
            match result {
                Ok(message) => {
                    if let Some(payload) = message.payload_view::<str>() {
                        match payload {
                            Ok(text) => {
                                println!("Received message: {}", text);
                            }
                            Err(e) => {
                                eprintln!("Error while deserializing message payload: {:?}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error while receiving from Kafka: {:?}", e);
                }
            }
        }
    });

    tokio::signal::ctrl_c().await.unwrap();

    println!("Hello, world!");
}
