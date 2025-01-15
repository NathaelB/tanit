use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::domain::car::{models::{Car, CarError}, ports::CarRepository};


#[derive(Clone, Default)]
pub struct InMemoryCarRepository {
  cars: Arc<Mutex<HashMap<String, Car>>>
}

impl CarRepository for InMemoryCarRepository {
  async fn create(&self, car: Car, ferry_id: String) -> Result<Car, CarError> {
      let mut cars = self.cars.lock().unwrap();
      cars.insert(ferry_id, car.clone());

      Ok(car)
  }

  async fn delete_by_ferry_id(&self, id: &str) -> Result<(), CarError> {
      let mut cars = self.cars.lock().unwrap();

      cars.remove(id);

      Ok(())
  }
}