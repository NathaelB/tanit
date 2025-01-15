use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::domain::passenger::{models::Passenger, ports::PassengerRepository};

#[derive(Clone, Default)]
pub struct InMemoryPassengerRepository {
  pub passengers: Arc<Mutex<HashMap<String, Vec<Passenger>>>>
}

impl PassengerRepository for InMemoryPassengerRepository {
  async fn save(&self, passenger: Passenger) -> anyhow::Result<()> {
      let mut passengers = self.passengers.lock().unwrap();
      let ferry_id = passenger.ferry_id.clone();
      let passenger_list = passengers.entry(ferry_id).or_insert(Vec::new());
      passenger_list.push(passenger.clone());

      Ok(())
  }
}