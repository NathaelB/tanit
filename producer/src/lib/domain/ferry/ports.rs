use std::future::Future;

use anyhow::Result;

use super::models::Ferry;

pub trait FerryService: Clone + Send + Sync + 'static {
    fn create(&self, capacity: i32) -> impl Future<Output = Result<Ferry>> + Send;
}
