use crate::application::ports::{MessagingPort, Offset, SubscriptionOptions};
use anyhow::Result;
use futures::StreamExt;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    producer::FutureProducer,
    ClientConfig, Message,
};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct Kafka {
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

        todo!()
    }
}
