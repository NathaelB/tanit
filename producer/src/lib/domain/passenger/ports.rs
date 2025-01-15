use std::future::Future;

use anyhow::Result;

use super::models::Passenger;

pub trait PassengerService: Clone + Send + Sync + 'static {
    fn create(
        &self,
        car_id: Option<String>,
        ferry_id: String,
    ) -> impl Future<Output = Result<Passenger>> + Send;
}
