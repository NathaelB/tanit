use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tanit::{
    application::{
        http::{HttpServer, HttpServerConfig},
        ports::{MessagingPort, SubscriptionOptions},
    },
    domain::ferri::{
        models::{CreateFerryEvent, Ferri},
        ports::FerriService,
        service::FerriServiceImpl,
    },
    infrastructure::{
        messaging::kafka::Kafka, repositories::in_memory_ferri_repository::InMemoryFerriRepository,
    },
};

use anyhow::Result;
use tracing::info;

pub async fn start_subscriptions<F>(messaging: Arc<Kafka>, ferri_service: Arc<F>) -> Result<()>
where
    F: FerriService,
{
    let messaging = Arc::clone(&messaging);
    let options = SubscriptionOptions {
        offset: tanit::application::ports::Offset::Beginning,
    };

    messaging
        .subscribe("ferris", "merger", options, {
            let ferri_service = Arc::clone(&ferri_service);

            move |e: CreateFerryEvent| {
                let ferri_service = Arc::clone(&ferri_service);

                info!("Received ferry: {:?}", e);

                async move {
                    ferri_service.add_ferry(Ferri::from_event(e)).await?;
                    Ok(())
                }
            }
        })
        .await?;
    
        messaging
        .subscribe("cars", "merger", options, {
            let ferri_service = Arc::clone(&ferri_service);
            move |c: CreateCarEvent| {
                info!("Received car: {:?}", c);
                async move {
                    ferri_service.add_car(Car::from_event(c)).await?;
                    Ok(())
                }
            }
        })
        .await?;

    messaging
        .subscribe("passenger", "merger", options, {
            let ferri_service = Arc::clone(&ferri_service);
            move |p: CreatePassengerEvent| {
                info!("Received passenger: {:?}", p);
                async move {
                    ferri_service.add_passenger(Passenger::from_event(p)).await?;
                    Ok(())
                }
            }
        })
        .await?;


    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let kafka = Arc::new(Kafka::new(
        "localhost:8098,localhost:8097".to_string(),
        "example_group".to_string(),
    )?);

    let ferri_repository = InMemoryFerriRepository::new();
    let ferri_service = Arc::new(FerriServiceImpl::new(ferri_repository));

    start_subscriptions(Arc::clone(&kafka), Arc::clone(&ferri_service)).await?;

    let server_config = HttpServerConfig::new("3333".to_string());

    let http_server = HttpServer::new(server_config, Arc::clone(&ferri_service)).await?;

    http_server.run().await?;

    Ok(())
}
