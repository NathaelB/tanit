use datafusion::{
    arrow::{
        array::{Int32Array, ListArray, ListBuilder, RecordBatch, StringArray, StringBuilder},
        datatypes::{DataType, Field, Schema},
    }, logical_expr::test, prelude::SessionContext
};
use std::sync::Arc;

use datafusion::dataframe::DataFrameWriteOptions;
use futures::StreamExt;

use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    ClientConfig, Message,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct FerryEvent {
    ferry_id: String,        // ID du ferry
    passengers: Vec<String>, // Liste des passagers
    cars: Vec<String>,       // Liste des voitures
    capacity: i32,           // Capacité du ferry
}

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "test-group")
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .unwrap();

    consumer.subscribe(&["test"]).unwrap();

    let schema = Arc::new(Schema::new(vec![
        Field::new("ferry_id", DataType::Utf8, false), // ID du ferry
        Field::new(
            "passengers",
            DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))),
            false,
        ), // Liste de passagers
        Field::new(
            "cars",
            DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))),
            false,
        ), // Liste de voitures
        Field::new("capacity", DataType::Int32, false), // Capacité du ferry
    ]));
    let mut ctx = SessionContext::new();

    let mut events: Vec<FerryEvent> = Vec::new();
    println!("Consumer loop started");
    tokio::spawn(async move {
        while let Some(result) = consumer.stream().next().await {
            match result {
                Ok(message) => {
                    if let Some(payload) = message.payload_view::<str>() {
                        match payload {
                            Ok(text) => {
                                let event: FerryEvent = serde_json::from_str(text)
                                    .expect("Impossible de désérialiser le message Kafka");

                                println!("Received message: {}", text);
                                events.push(event);

                                if events.len() >= 10 {
                                    let ferry_ids: Vec<String> =
                                        events.iter().map(|e| e.ferry_id.clone()).collect();
                                    let capacities: Vec<i32> =
                                        events.iter().map(|e| e.capacity).collect();
                                    let passengers: Vec<Vec<String>> =
                                        events.iter().map(|e| e.passengers.clone()).collect();
                                    let cars: Vec<Vec<String>> =
                                        events.iter().map(|e| e.cars.clone()).collect();
                                    let ferry_id_array = StringArray::from(ferry_ids);
                                    let capacity_array = Int32Array::from(capacities);
                                    let passengers_array = ListArray::from(create_list_array(passengers));
                                    let cars_array = ListArray::from(create_list_array(cars));
                        
                                    // creer dataframe
                                    let batch = RecordBatch::try_new(
                                        schema.clone(),
                                        vec![Arc::new(ferry_id_array), Arc::new(capacity_array),Arc::new(passengers_array), Arc::new(cars_array),],
                                    )
                                    .unwrap();

                                    ctx.register_batch("ferry_events", batch);
                                    let df = ctx.table("ferry_events").await.unwrap();

                                    df.clone().show().await.unwrap();
                                    let parquet_path = "./output/ferry_events.parquet";

                                    // parquer le dataframe
                                    df.write_parquet(
                                        parquet_path,
                                        DataFrameWriteOptions::new(),
                                        None,
                                    )
                                    .await
                                    .unwrap();

                                    events.clear();
                                }
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

fn create_list_array(data: Vec<Vec<String>>) -> ListArray {
    let mut builder = ListBuilder::new(StringBuilder::new());
    for list in data {
        let string_builder = builder.values();
        for value in list {
            string_builder.append_value(value);
        }
        builder.append(true);
    }
    builder.finish()
}