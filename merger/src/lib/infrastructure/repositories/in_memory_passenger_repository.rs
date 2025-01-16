use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tracing::info;

use crate::domain::passenger::{models::Passenger, ports::PassengerRepository};

#[derive(Clone, Default)]
pub struct InMemoryPassengerRepository {
    pub passengers: Arc<Mutex<HashMap<String, Vec<Passenger>>>>,
}

impl PassengerRepository for InMemoryPassengerRepository {
    async fn save(&self, passenger: Passenger) -> anyhow::Result<()> {
        let mut passengers = self.passengers.lock().unwrap();
        let ferry_id = passenger.ferry_id.clone();
        let passenger_list = passengers.entry(ferry_id.clone()).or_insert(Vec::new());
        let passenger_nb = passenger_list.len();

        info!(
            "Il y a {} passagers dans le ferry {}",
            passenger_nb, ferry_id
        );
        passenger_list.push(passenger.clone());

        Ok(())
    }

    async fn find_capacity_by_ferry_id(&self, ferry_id: String) -> anyhow::Result<usize> {
        let passengers = self.passengers.lock().unwrap();
        let passenger_list = passengers.get(&ferry_id).unwrap();
        let passenger_nb = passenger_list.len();

        Ok(passenger_nb)
    }
}
