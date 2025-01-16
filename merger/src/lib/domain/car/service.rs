use super::{
    models::{Car, CarError},
    ports::{CarRepository, CarService},
};

#[derive(Clone)]
pub struct CarServiceImpl<C>
where
    C: CarRepository,
{
    pub car_repository: C,
}

impl<C> CarServiceImpl<C>
where
    C: CarRepository,
{
    pub fn new(car_repository: C) -> Self {
        Self { car_repository }
    }
}

impl<C> CarService for CarServiceImpl<C>
where
    C: CarRepository,
{
    async fn create(&self, car: Car) -> Result<Car, CarError> {
        self.car_repository.create(car).await
    }

    async fn find_from_ids(&self, ids: Vec<String>) -> Result<Vec<Car>, CarError> {
        self.car_repository.find_from_ids(ids).await
    }

    async fn delete_by_ids(&self, ids: Vec<String>) -> Result<Vec<Car>, CarError> {
        self.car_repository.delete_by_ids(ids).await
    }
}
