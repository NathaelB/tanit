use std::collections::HashMap;

use anyhow::Result;
use fake::faker::automotive::fr_fr::LicencePlate;
use fake::faker::company::en::CompanyName;
use fake::faker::lorem::en::Word;
use fake::faker::name::en::{FirstName, LastName};
use fake::Fake;
use rand::Rng;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;
use schema_registry_converter::async_impl::avro::AvroEncoder;
use tanit::application::http::{HttpServer, HttpServerConfig};
use tanit::application::messaging::{create_car_schema, create_ferri_schema, create_passernger_schema};
use tanit::domain::models::{Car, Ferri, Passenger};
use uuid::Uuid;

use apache_avro::{types::Value, Schema, Writer};
use schema_registry_converter::async_impl::schema_registry::{post_schema, SrSettings};
use schema_registry_converter::schema_registry_common::{
    SchemaType, SubjectNameStrategy, SuppliedSchema,
};

use tanit::application::ports::MessagingPort;
use tanit::infrastructure::messaging::kafka::Kafka;

fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

fn generate_random_ferry() -> Ferry {
    Ferry {
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

fn generate_random_passenger(ferry_id: &str, car_id: Option<&str>) -> Passenger {
    let mut rng = rand::thread_rng();
    Passenger {
        id: generate_id(),
        car_id: car_id.map(|id| id.to_string()),
        ferry_id: ferry_id.to_string(),
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
async fn main() -> Result<()> {
    println!("Hello, world!");


    create_car_schema().await?;
    create_ferri_schema().await?;
    create_passernger_schema().await?;

    // let sr_settings = SrSettings::new("http://localhost:8081".to_string());

    // let mut record = HashMap::new();
    // record.insert("id".to_string(), Value::String("123".to_string()));
    // record.insert("name".to_string(), Value::String("Ferry".to_string()));
    // record.insert("capacity".to_string(), Value::Int(500));

    // // Vec<(&str, Value)>
    // let vec_value = vec![
    //     ("id", Value::String("123".to_string())),
    //     ("name", Value::String("Ferry".to_string())),
    //     ("capacity", Value::Int(500)),
    // ];

    // println!("logs: create value & record");

    // let avro_encoder = AvroEncoder::new(sr_settings);
    // // let subject_name_strategy = SubjectNameStrategy::TopicNameStrategyWithSchema(
    // //     String::from("ferris"),
    // //     false,
    // //     supplied_schema,
    // // );

    // let subject_name_strategy =
    //     SubjectNameStrategy::TopicNameStrategy(String::from("ferris"), false);

    // println!("logs: create avro encoder");

    // let record_vec = vec![record];
    // println!("logs: create record_vec");
    // println!("record_vec: {:?}", record_vec);
    // let encoded_message = match avro_encoder.encode(vec_value, subject_name_strategy).await {
    //     Ok(msg) => {
    //         println!("Encoded message: {:?}", msg);
    //         msg
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to encode message: {:?}", e);
    //         return Err(e.into());
    //     }
    // };

    // println!("Encoded message: {:?}", encoded_message);

    // let producer: FutureProducer = ClientConfig::new()
    //     .set("bootstrap.servers", "localhost:19092")
    //     .create()?;

    // let delivery_status = producer
    //     .send(
    //         FutureRecord::to("ferris")
    //             .payload(&encoded_message)
    //             .key("5"),
    //         Timeout::Never,
    //     )
    //     .await;

    // match delivery_status {
    //     Ok(delivery) => println!("Delivery status: {:?}", delivery),
    //     Err((e, _)) => eprintln!("Failed to deliver message: {:?}", e),
    // }

    // let sr_settings = SrSettings::new("http://localhost:8081".to_string());
    // let client = SchemaRegistryClient::new(sr_settings);

    // // Parsing du schéma Avro
    // let schema = Schema::parse_str(schema_str)?;

    // // Stratégie de nom de sujet
    // let subject_name_strategy = SubjectNameStrategy::TopicNameStrategy("test-topic".to_string(), false);

    // // Enregistrement du schéma
    // let schema_id = client.register(&subject_name_strategy, &schema).await?;

    // // Affichage de l'ID du schéma enregistré
    // println!("Schema registered with ID: {}", schema_id);

    //generate_random_data();

    let server_config = HttpServerConfig::new("3333".to_string());
    let http_server = HttpServer::new(server_config).await?;

    http_server.run().await?;

    Ok(())
}
