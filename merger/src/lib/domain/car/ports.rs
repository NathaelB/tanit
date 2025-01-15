use std::future::Future;

use super::models::{Car, CarError};

pub trait CarService: Clone + Send + Sync + 'static {
    fn create(&self, car: Car) -> impl Future<Output = Result<Car, CarError>> + Send;
}

pub trait CarRepository: Clone + Send + Sync + 'static {
    fn create(&self, car: Car) -> impl Future<Output = Result<Car, CarError>> + Send;
    fn delete_by_id(&self, id: &str) -> impl Future<Output = Result<(), CarError>> + Send;
}

