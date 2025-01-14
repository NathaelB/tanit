use anyhow::Result;
use std::future::Future;
use std::sync::Arc;

use super::models::Car;
use super::models::DataSet;
use super::models::Ferry;
use super::models::Passenger;

pub trait FerryService: Clone + Send + Sync + 'static {
    fn create(&self, capacity: i32) -> impl Future<Output = Result<Ferry>> + Send;
}

pub trait CarService: Clone + Send + Sync + 'static {
    fn create(&self) -> impl Future<Output = Result<Car>> + Send;
}

pub trait PassengerService: Clone + Send + Sync + 'static {
    fn create(
        &self,
        car_id: Option<String>,
        ferry_id: String,
    ) -> impl Future<Output = Result<Passenger>> + Send;
}

pub trait DataSetService: Clone + Send + Sync + 'static {
    fn create_data_set<F: FerryService, C: CarService, P: PassengerService>(
        &self,
        capacity_ferry: i32,
        ferry_service: Arc<F>,
        car_service: Arc<C>,
        passenger_service: Arc<P>,
    ) -> impl Future<Output = Result<DataSet>> + Send;
}
