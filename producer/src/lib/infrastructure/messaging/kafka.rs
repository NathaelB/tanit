use crate::application::ports::{MessagingPort, Offset, SubscriptionOptions};
use anyhow::Result;
use futures::StreamExt;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    producer::{FutureProducer, FutureRecord},
    ClientConfig, Message,
};
use schema_registry_converter::{
    async_impl::{avro::AvroEncoder, schema_registry::SrSettings},
    schema_registry_common::SubjectNameStrategy,
};
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};

pub enum SchemaType {
    Ferry,
    Car,
    Passenger,
}

#[derive(Clone)]
pub struct Kafka {
    #[allow(unused)]
    producer: Arc<FutureProducer>,
    consumer: Arc<StreamConsumer>,
}

impl Kafka {
    pub fn new(brokers: String, group_id: String) -> Result<Self> {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .create::<FutureProducer>()?;

        let consumer = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .set("group.id", &group_id)
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "earliest")
            .create::<StreamConsumer>()?;

        Ok(Kafka {
            producer: Arc::new(producer),
            consumer: Arc::new(consumer),
        })
    }
}

impl MessagingPort for Kafka {
    async fn publish_message<T: Serialize>(&self, topic: String, message: T) -> anyhow::Result<()> {
        let producer = Arc::clone(&self.producer);

        let sr_settings = SrSettings::new("http://localhost:8081".to_string());
        let encoder = AvroEncoder::new(sr_settings);

        let subject_name_strategy = SubjectNameStrategy::TopicNameStrategy(topic.clone(), false);
        let encoded_message = encoder
            .encode_struct(message, &subject_name_strategy)
            .await?;

        let record = FutureRecord::to(&topic)
            .payload(&encoded_message)
            .key("key");

        producer
            .send(record, rdkafka::util::Timeout::Never)
            .await
            .map_err(|(e, _)| anyhow::Error::new(e))?;

        Ok(())
    }

    async fn subscribe<F, T, Fut>(
        &self,
        topic: &str,
        _group_id: &str,
        options: SubscriptionOptions,
        handler: F,
    ) -> anyhow::Result<()>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
        T: serde::de::DeserializeOwned + Send + Sync + std::fmt::Debug + Clone + 'static,
    {
        self.consumer.subscribe(&[topic])?;

        if let Offset::Beginning = options.offset {
            let mut hash_map = HashMap::new();
            hash_map.insert((topic.to_string(), 0), rdkafka::Offset::Beginning);
            let t = rdkafka::TopicPartitionList::from_topic_map(&hash_map).unwrap();
            self.consumer.assign(&t)?;
        }

        let consumer: Arc<StreamConsumer> = Arc::clone(&self.consumer);

        tokio::spawn(async move {
            while let Some(result) = consumer.stream().next().await {
                match result {
                    Ok(message) => {
                        if let Some(payload) = message.payload_view::<str>() {
                            match payload {
                                Ok(text) => {
                                    let parsed_message: T = match serde_json::from_str(text) {
                                        Ok(msg) => msg,
                                        Err(e) => {
                                            eprintln!("Failed to parse message: {:?}", e);
                                            continue;
                                        }
                                    };

                                    if let Err(e) = handler(parsed_message).await {
                                        eprintln!("Failed to handle message: {:?}", e);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error while reading message: {:?}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error while reading from stream: {:?}", e);
                    }
                }
            }
        });

        Ok(())
    }
}
