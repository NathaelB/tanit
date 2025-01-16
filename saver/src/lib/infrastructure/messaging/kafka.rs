use crate::application::ports::{MessagingPort, Offset, SubscriptionOptions};
use anyhow::Result;
use apache_avro::from_value;
use futures::StreamExt;
use rdkafka::{consumer::{Consumer, StreamConsumer}, producer::FutureProducer, ClientConfig, Message};
use schema_registry_converter::async_impl::{avro::AvroDecoder, schema_registry::SrSettings};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct Kafka {
    #[allow(unused)]
    producer: Arc<FutureProducer>,
    brokers: String,
}

impl Kafka {
    pub fn new(brokers: String, _group_id: String) -> Result<Self> {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .create::<FutureProducer>()?;

        Ok(Kafka {
            producer: Arc::new(producer),
            brokers,
        })
    }
}

impl MessagingPort for Kafka {
    async fn publish_message(&self, _topic: String, _message: String) -> anyhow::Result<()> {
        todo!()
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
        let consumer = ClientConfig::new()
            .set("bootstrap.servers", &self.brokers)
            .set("group.id", "merger")
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "earliest")
            .create::<StreamConsumer>()?;

        // if let Offset::Beginning = options.offset {
        //     let mut hash_map = HashMap::new();
        //     hash_map.insert((topic.to_string(), 0), rdkafka::Offset::Beginning);
        //     let t = rdkafka::TopicPartitionList::from_topic_map(&hash_map).unwrap();
        //     consumer.assign(&t)?;
        // }

        consumer.subscribe(&[topic])?;

        let sr_settings = SrSettings::new("http://localhost:8081".to_string());
        let avro_decoder = AvroDecoder::new(sr_settings);

        tokio::spawn(async move {
            while let Some(result) = consumer.stream().next().await {
                match result {
                    Ok(message) => {
                        if let Some(payload) = message.payload_view::<[u8]>() {
                            match payload {
                                Ok(bytes) => {
                                    let decoded =
                                        match avro_decoder.decode_with_schema(Some(bytes)).await {
                                            Ok(decoded) => decoded,
                                            Err(e) => {
                                                eprintln!("Error while decoding message: {:?}", e);
                                                continue;
                                            }
                                        }
                                        .unwrap();
                                    let parsed_message: T = match from_value(&decoded.value) {
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
