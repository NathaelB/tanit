use anyhow::Result;
use datafusion::{
    arrow::{
        array::{Int32Array, ListArray, ListBuilder, RecordBatch, StringArray, StringBuilder},
        datatypes::{DataType, Field, Schema},
    },
    prelude::SessionContext,
};
use std::sync::Arc;
use tanit::{
    application::ports::{MessagingPort, SubscriptionOptions},
    infrastructure::messaging::kafka::Kafka,
};
use tokio::sync::RwLock;

use datafusion::dataframe::DataFrameWriteOptions;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct FerryEvent {
    ferry_id: String,        // ID du ferry
    passengers: Vec<String>, // Liste des passagers
    cars: Vec<String>,       // Liste des voitures
    capacity: i32,           // Capacité du ferry
}

#[tokio::main]
async fn main() -> Result<()> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("ferry_id", DataType::Utf8, false),
        Field::new(
            "passengers",
            DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))),
            false,
        ),
        Field::new(
            "cars",
            DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))),
            false,
        ),
        Field::new("capacity", DataType::Int32, false),
    ]));
    let ctx = SessionContext::new();

    let events: Arc<RwLock<Vec<FerryEvent>>> = Arc::new(RwLock::new(Vec::new()));

    let kafka = Arc::new(
        Kafka::new(
            "localhost:8098,localhost:8097".to_string(),
            "example_group".to_string(),
        )
        .unwrap(),
    );

    start_subscriptions(Arc::clone(&kafka), schema, events, Arc::new(ctx)).await?;
    Ok(())
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

pub async fn start_subscriptions(
    messaging: Arc<Kafka>,
    schema: Arc<Schema>,
    events: Arc<RwLock<Vec<FerryEvent>>>,
    ctx: Arc<SessionContext>,
) -> Result<()> {
    let messaging = Arc::clone(&messaging);
    let options = SubscriptionOptions {
        offset: tanit::application::ports::Offset::Beginning,
    };

    messaging
        .subscribe("ferris", "saver", options, {
            let s = Arc::clone(&schema);
            let ctx = Arc::clone(&ctx);
            let events = Arc::clone(&events);

            move |e: FerryEvent| {
                let es1 = events.clone();
                let schema = s.clone();
                let ctx = ctx.clone();

                async move {
                    let mut events_guard = es1.write().await;
                    events_guard.push(e.clone());

                    if events_guard.len() >= 10 {
                        let result: Result<(), anyhow::Error> = async {
                            let ferry_ids: Vec<String> =
                                events_guard.iter().map(|e| e.ferry_id.clone()).collect();
                            let capacities: Vec<i32> =
                                events_guard.iter().map(|e| e.capacity).collect();
                            let passengers: Vec<Vec<String>> =
                                events_guard.iter().map(|e| e.passengers.clone()).collect();
                            let cars: Vec<Vec<String>> =
                                events_guard.iter().map(|e| e.cars.clone()).collect();
                            let ferry_id_array = StringArray::from(ferry_ids);
                            let capacity_array = Int32Array::from(capacities);
                            let passengers_array = ListArray::from(create_list_array(passengers));
                            let cars_array = ListArray::from(create_list_array(cars));
                            let batch = RecordBatch::try_new(
                                schema.clone(),
                                vec![
                                    Arc::new(ferry_id_array),
                                    Arc::new(capacity_array),
                                    Arc::new(passengers_array),
                                    Arc::new(cars_array),
                                ],
                            )?;

                            ctx.register_batch("ferry_events", batch)?;
                            let df = ctx.table("ferry_events").await?;

                            df.clone().show().await?;
                            let parquet_path = "./output/ferry_events.parquet";

                            // parquer le dataframe
                            df.write_parquet(parquet_path, DataFrameWriteOptions::new(), None)
                                .await?;
                            Ok(())
                            // events_guard.push(e.clone());
                        }
                        .await;
                        match result {
                            Ok(_) => {
                                events_guard.clear();
                            }
                            Err(e) => {
                                eprintln!("Error: {:?}", e);
                            }
                        };
                    };
                    Ok(())
                }
            }
        })
        .await?;

    Ok(())
}
