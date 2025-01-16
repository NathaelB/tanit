use std::sync::Arc;
use tanit::{
    application::{
        http::{HttpServer, HttpServerConfig},
        messaging::create_saver_schema,
        ports::{MessagingPort, Offset, SubscriptionOptions},
    },
    domain::{
        car::{
            models::{Car, CreateCarEvent},
            ports::CarService,
            service::CarServiceImpl,
        },
        ferri::{
            models::{CreateFerryEvent, Ferri},
            ports::FerriService,
            service::FerriServiceImpl,
        },
        passenger::{
            models::{CreatePassengerEvent, Passenger},
            ports::PassengerService,
            service::PassengerServiceImpl,
        },
    },
    infrastructure::{
        messaging::kafka::Kafka,
        repositories::{
            in_memory_car_repository::InMemoryCarRepository,
            in_memory_ferri_repository::InMemoryFerriRepository,
            in_memory_passenger_repository::InMemoryPassengerRepository,
        },
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
        .subscribe("ferries", "merger", options, {
            info!("listen to ferri");
            move |e: CreateFerryEvent| {
                let ferri_service = Arc::clone(&ferri_service);
                async move {
                    ferri_service.add_ferry(Ferri::from_event(e)).await?;
                    Ok(())
                }
            }
        })
        .await?;

    // attendre 5sec
    //tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    messaging
        .subscribe(
            "cars",
            "merger",
            SubscriptionOptions {
                offset: Offset::Latests,
            },
            {
                info!("listen to cars");
                move |c: CreateCarEvent| {
                    let car_service = Arc::clone(&car_service);

                    async move {
                        car_service.create(Car::from_event(c)).await?;
                        Ok(())
                    }
                }
            },
        )
        .await?;

    messaging
        .subscribe(
            "passengers",
            "merger",
            SubscriptionOptions {
                offset: Offset::Latests,
            },
            {
                info!("listen to passenger");
                move |p: CreatePassengerEvent| {
                    let passenger_service = Arc::clone(&passenger_service);

                    async move {
                        passenger_service
                            .add_passenger(Passenger::from_event(p))
                            .await?;
                        Ok(())
                    }
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

    create_saver_schema().await?;

    let car_repository = InMemoryCarRepository::default();
    let ferri_repository = InMemoryFerriRepository::new();
    let passenger_repository = InMemoryPassengerRepository::default();

    let car_service = Arc::new(CarServiceImpl::new(car_repository));

    let ferri_service = Arc::new(FerriServiceImpl::new(
        ferri_repository,
        Arc::clone(&car_service),
        Arc::clone(&kafka),
    ));

    let passenger_service = Arc::new(PassengerServiceImpl::new(
        passenger_repository,
        Arc::clone(&ferri_service),
    ));

    start_subscriptions(
        Arc::clone(&kafka),
        Arc::clone(&ferri_service),
        Arc::clone(&car_service),
        Arc::clone(&passenger_service),
    )
    .await?;

    let server_config = HttpServerConfig::new("3333".to_string());

    let http_server = HttpServer::new(
        server_config,
        Arc::clone(&ferri_service),
        Arc::clone(&car_service),
        Arc::clone(&passenger_service),
    )
    .await?;

    http_server.run().await?;

    Ok(())
}
