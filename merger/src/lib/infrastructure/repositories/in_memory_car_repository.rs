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
}
