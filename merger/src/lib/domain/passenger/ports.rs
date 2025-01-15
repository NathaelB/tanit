use std::future::Future;

use super::models::Passenger;

pub trait PassengerService: Clone + Send + Sync + 'static {
    fn add_passenger(
        &self,
        passenger: Passenger,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

pub trait PassengerRepository: Clone + Send + Sync + 'static {
    fn save(&self, passenger: Passenger) -> impl Future<Output = anyhow::Result<()>> + Send;
}
