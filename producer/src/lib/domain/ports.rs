use anyhow::Result;
use std::future::Future;
use std::sync::Arc;

use super::car::ports::CarService;
use super::ferry::ports::FerryService;
use super::models::DataSet;
use super::passenger::ports::PassengerService;

pub trait DataSetService: Clone + Send + Sync + 'static {
    fn create_data_set<F: FerryService, C: CarService, P: PassengerService>(
        &self,
        capacity_ferry: i32,
        ferry_service: Arc<F>,
        car_service: Arc<C>,
        passenger_service: Arc<P>,
    ) -> impl Future<Output = Result<DataSet>> + Send;
}
