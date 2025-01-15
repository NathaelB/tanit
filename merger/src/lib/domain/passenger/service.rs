use std::future::Future;

use super::{
    models::Passenger,
    ports::{PassengerRepository, PassengerService},
};

#[derive(Clone)]
pub struct PassengerServiceImpl<P>
where
    P: PassengerRepository,
{
    pub passenger_repository: P,
}

impl<P> PassengerServiceImpl<P>
where
    P: PassengerRepository,
{
    pub fn new(passenger_repository: P) -> Self {
        Self {
            passenger_repository,
        }
    }
}

impl<P> PassengerService for PassengerServiceImpl<P>
where
    P: PassengerRepository,
{
    fn add_passenger(
        &self,
        passenger: Passenger,
    ) -> impl Future<Output = anyhow::Result<()>> + Send {
        self.passenger_repository.save(passenger)
    }
}
