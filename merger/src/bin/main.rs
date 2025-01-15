use std::sync::Arc;
use tanit::{
    application::{
        http::{HttpServer, HttpServerConfig},
        ports::{MessagingPort, Offset, SubscriptionOptions},
    },
    domain::{
        car::{models::CreateCarEvent, ports::CarService},
        ferri::{
            models::{CreateFerryEvent, Ferri},
            ports::FerriService,
            service::FerriServiceImpl,
        },
        passenger::{models::CreatePassengerEvent, ports::PassengerService},
    },
    infrastructure::{
        messaging::kafka::Kafka, repositories::in_memory_ferri_repository::InMemoryFerriRepository,
    },
};

use anyhow::Result;
use tracing::info;

pub async fn start_subscriptions<F, C, P>(
    messaging: Arc<Kafka>,
    ferri_service: Arc<F>,
    car_service: Arc<C>,
    passenger_service: Arc<P>,
) -> Result<()>
where
    F: FerriService,
    C: CarService,
    P: PassengerService,
{
    let messaging = Arc::clone(&messaging);
    let options = SubscriptionOptions {
        offset: tanit::application::ports::Offset::Beginning,
    };

    messaging
        .subscribe("ferris", "merger", options, {
            let _ferri_service = Arc::clone(&ferri_service);

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
        .subscribe(
            "cars",
            "merger",
            SubscriptionOptions {
                offset: Offset::Latests,
            },
            {
                move |c: CreateCarEvent| {
                    info!("Received car: {:?}", c);
                    async move { Ok(()) }
                }
            },
        )
        .await?;

    messaging
        .subscribe(
            "passenger",
            "merger",
            SubscriptionOptions {
                offset: Offset::Latests,
            },
            {
                move |p: CreatePassengerEvent| {
                    info!("Received passenger: {:?}", p);
                    async move { Ok(()) }
                }
            },
        )
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let kafka = Arc::new(Kafka::new(
        String::from("localhost:19092"),
        "example_group".to_string(),
    )?);

    let ferri_repository = InMemoryFerriRepository::new();
    let ferri_service = Arc::new(FerriServiceImpl::new(ferri_repository));


    //start_subscriptions(Arc::clone(&kafka), Arc::clone(&ferri_service)).await?;

    let server_config = HttpServerConfig::new("3333".to_string());

    let http_server = HttpServer::new(server_config, Arc::clone(&ferri_service)).await?;

    http_server.run().await?;

    Ok(())
}
