use std::future::Future;

use anyhow::Result;

use super::models::Car;

pub trait CarService: Clone + Send + Sync + 'static {
    fn create(&self) -> impl Future<Output = Result<Car>> + Send;
}
