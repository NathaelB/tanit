use anyhow::Result;
use fake::faker::automotive::fr_fr::LicencePlate;
use fake::faker::company::en::CompanyName;
use fake::faker::lorem::en::Word;
use fake::faker::name::en::{FirstName, LastName};
use fake::Fake;
use rand::Rng;
use serde::Serialize;
use tanit::domain::models::{Car, Ferri, Passenger};
use uuid::Uuid;

use tanit::application::ports::MessagingPort;
use tanit::infrastructure::messaging::kafka::Kafka;

fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

fn generate_random_ferri() -> Ferri {
    Ferri {
        id: generate_id(),
        name: CompanyName().fake(),
        capacity: rand::thread_rng().gen_range(50..200),
    }
}

fn generate_random_car() -> Car {
    let mut rng = rand::thread_rng();
    Car {
        id: generate_id(),
        licence_plate: LicencePlate().fake(),
        brand: Word().fake(),
        color: Word().fake(),
        capacity: rng.gen_range(2..5),
    }
}

fn generate_random_passenger(ferri_id: &str, car_id: Option<&str>) -> Passenger {
    let mut rng = rand::thread_rng();
    Passenger {
        id: generate_id(),
        car_id: car_id.map(|id| id.to_string()),
        ferri_id: ferri_id.to_string(),
        firstname: FirstName().fake(),
        lastname: LastName().fake(),
        sex: rng.gen_bool(0.5),
    }
}

fn send_to_kafka<T: Serialize>(host: &str, topic: String, data: &T) {
    // let mut producer = Producer::from_hosts(vec![host.to_owned()])
    //     .with_ack_timeout(Duration::from_secs(1))
    //     .with_required_acks(RequiredAcks::One)
    //     .create()
    //     .unwrap();

    // let payload = serde_json::to_string(data).unwrap();

    // producer
    //     .send(&Record::from_value(topic, payload.as_bytes()))
    //     .unwrap();

    // Create a Kafka instance
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Kafka configuration
    let kafka_host = "localhost:19092";

    for _i in 0..10 {
        // Generate random ferry
        let ferri = generate_random_ferri();
        send_to_kafka(kafka_host, "ferries".to_string(), &ferri);

        for _j in 0..30 {
            // Generate random car
            let car = generate_random_car();
            send_to_kafka(kafka_host, "cars".to_string(), &car);

            for _k in 0..5 {
                // Generate random passengers
                let passenger_with_car = generate_random_passenger(&ferri.id, Some(&car.id));
                send_to_kafka(kafka_host, "passengers".to_string(), &passenger_with_car);
            }
        }

        for _k in 0..10 {
            let passenger_without_car = generate_random_passenger(&ferri.id, None);
            send_to_kafka(kafka_host, "passengers".to_string(), &passenger_without_car);
        }
    }

    Ok(())
}
