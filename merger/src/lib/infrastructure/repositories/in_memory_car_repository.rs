use std::sync::{Arc, Mutex};

use crate::domain::car::{
    models::{Car, CarError},
    ports::CarRepository,
};

#[derive(Clone, Default)]
pub struct InMemoryCarRepository {
    cars: Arc<Mutex<Vec<Car>>>,
}

impl CarRepository for InMemoryCarRepository {
    async fn create(&self, car: Car) -> Result<Car, CarError> {
        let mut cars = self.cars.lock().unwrap();
        cars.push(car.clone());

        Ok(car)
    }

    async fn find_from_ids(&self, ids: Vec<String>) -> Result<Vec<Car>, CarError> {
        let cars = self.cars.lock().unwrap();
        let cars = cars
            .iter()
            .filter(|c| ids.contains(&c.id))
            .cloned()
            .collect();

        Ok(cars)
    }

    async fn delete_by_ids(&self, ids: Vec<String>) -> Result<Vec<Car>, CarError> {
        let mut cars = self.cars.lock().unwrap();
        let mut deleted_cars = Vec::new();
        cars.retain(|c| {
            if ids.contains(&c.id) {
                deleted_cars.push(c.clone());
                false
            } else {
                true
            }
        });

        Ok(deleted_cars)
    }
}
