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
        let passenger_list = passengers.entry(ferry_id.clone()).or_default();
        let passenger_nb = passenger_list.len();

        info!(
            "Il y a {} passagers dans le ferry {}",
            passenger_nb, ferry_id
        );
        passenger_list.push(passenger.clone());

        Ok(())
    }

    async fn find_capacity_by_ferry_id(&self, ferry_id: String) -> anyhow::Result<i32> {
        let passengers = self.passengers.lock().unwrap();
        // si le ferry n'existe pas, on retourne 0
        if !passengers.contains_key(&ferry_id) {
            return Ok(0);
        }

        let passenger_list = passengers.get(&ferry_id).unwrap();
        let passenger_nb = passenger_list.len() + 1;

        Ok(passenger_nb as i32)
    }

    async fn find_by_ferry_id(&self, ferry_id: &str) -> anyhow::Result<Vec<Passenger>> {
        let passengers = self.passengers.lock().unwrap();
        let passenger_list = passengers.get(ferry_id).unwrap();

        Ok(passenger_list.clone())
    }

    async fn delete_by_ferry_id(&self, ferry_id: &str) -> anyhow::Result<()> {
        let mut passengers = self.passengers.lock().unwrap();
        passengers.remove(ferry_id);

        Ok(())
    }
}
