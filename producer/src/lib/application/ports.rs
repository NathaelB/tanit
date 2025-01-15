use std::{fmt::Debug, future::Future};

use serde::de::DeserializeOwned;

#[derive(Clone, Debug)]
pub enum Offset {
    Beginning,
    Latests,
    Offset(i64),
}

#[derive(Clone, Debug)]
pub struct SubscriptionOptions {
    pub offset: Offset,
}

pub trait MessagingPort: Clone + Send + Sync + 'static {
    fn publish_message(
        &self,
        topic: String,
        message: String,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;

    fn subscribe<F, T, Fut>(
        &self,
        topic: &str,
        group_id: &str,
        options: SubscriptionOptions,
        handler: F,
    ) -> impl Future<Output = anyhow::Result<()>> + Send
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = anyhow::Result<()>> + Send + 'static,
        T: DeserializeOwned + Send + Sync + Debug + Clone + 'static;
}
