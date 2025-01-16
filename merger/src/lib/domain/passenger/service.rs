use std::{future::Future, sync::Arc};

use crate::domain::ferri::ports::FerriService;

use super::{
    models::Passenger,
    ports::{PassengerRepository, PassengerService},
};

#[derive(Clone)]
pub struct PassengerServiceImpl<P, F>
where
    P: PassengerRepository,
    F: FerriService,
{
    pub passenger_repository: P,
    pub ferry_service: Arc<F>
}

impl<P, F> PassengerServiceImpl<P, F>
where
    P: PassengerRepository,
    F: FerriService,
{
    pub fn new(passenger_repository: P, ferri_service: Arc<F>) -> Self {
        Self {
            passenger_repository,
            ferry_service: Arc::clone(&ferri_service),
        }
    }
}

impl<P, F> PassengerService for PassengerServiceImpl<P, F>
where
    P: PassengerRepository,
    F: FerriService,
{
    async fn add_passenger(&self, passenger: Passenger) -> anyhow::Result<()> {
        let capacity = self
            .passenger_repository
            .find_capacity_by_ferry_id(passenger.ferry_id.clone())
            .await?;

        let ferry = self.ferry_service.find_by_id(&passenger.ferry_id).await?;

        if let Some(ferry) = ferry {
            if capacity < ferry.capacity {
                return Err(anyhow::anyhow!("Ferry is full"));
            }
        }

       

        self.passenger_repository.save(passenger).await
    }
}
