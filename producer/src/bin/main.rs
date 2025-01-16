use anyhow::Result;
use clap::Parser;
use futures::future::join_all;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use tanit::application::http::{HttpServer, HttpServerConfig};
use tanit::application::messaging::{
    create_car_schema, create_ferri_schema, create_passenger_schema,
};
use tanit::application::ports::MessagingPort;
use tanit::domain::car::services::CarServiceImpl;
use tanit::domain::ferry::services::FerryServiceImpl;
use tanit::domain::passenger::services::PassengerServiceImpl;
use tanit::domain::ports::DataSetService;
use tanit::domain::services::DataSetServiceImpl;
use tanit::infrastructure::env::Env;
use tanit::infrastructure::messaging::kafka::Kafka;
use tokio::time::interval;

fn _send_to_kafka<T: Serialize>(host: &str, topic: String, data: &T) {
    let kafka = Kafka::new(host.to_string(), "default-group".to_string())
        .expect("Failed to initialize Kafka");

    let payload = serde_json::to_string(data).expect("Failed to serialize data");

    tokio::spawn(async move {
        if let Err(e) = kafka.publish_message(topic, payload).await {
            eprintln!("Error sending message to Kafka: {:?}", e);
        }
    });
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let env = Arc::new(Env::parse());

    let kafka = Kafka::new(
        String::from("localhost:19092"),
        String::from("default-group"),
    )
    .expect("Failed to initialize Kafka");

    let kafka = Arc::new(kafka);

    let ferry_service = Arc::new(FerryServiceImpl); // Ensure this is properly implemented.
    let car_service = Arc::new(CarServiceImpl); // Ensure this is properly implemented.
    let passenger_service = Arc::new(PassengerServiceImpl); // Ensure this is properly implemented.
    let dataset_service = Arc::new(DataSetServiceImpl::new(
        Arc::clone(&ferry_service),
        Arc::clone(&car_service),
        Arc::clone(&passenger_service),
        Arc::clone(&kafka),
    ));

    //dataset_service.produce_data(env.messages).await?;

    let mut interval = interval(Duration::from_secs(1));

    tokio::spawn(async move {
        loop {
            interval.tick().await;

            let mut tasks = Vec::new();

            for _ in 0..env.messages {
                let dataset_service_clone = Arc::clone(&dataset_service);

                let task =
                    tokio::spawn(async move { dataset_service_clone.create_ferry(50).await });
                tasks.push(task);
            }

            let results = join_all(tasks).await;

            for result in results {
                if let Err(e) = result {
                    eprintln!("Error producing data: {:?}", e);
                }
            }

            println!("Producing data");
        }
    })
    .await?;

    create_car_schema().await?;
    create_ferri_schema().await?;
    create_passenger_schema().await?;

    println!("Schemas created successfully");

    let server_config = HttpServerConfig::new("3333".to_string());
    let http_server = HttpServer::new(server_config).await?;

    http_server.run().await?;

    Ok(())
}
