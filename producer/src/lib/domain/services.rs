use super::car::ports::CarService;
use std::sync::Arc;

use super::ferry::ports::FerryService;

use super::passenger::ports::PassengerService;
use super::{models::DataSet, ports::DataSetService};

#[derive(Clone, Default)]
pub struct DataSetServiceImpl;

impl DataSetService for DataSetServiceImpl {
    async fn create_data_set<F: FerryService, C: CarService, P: PassengerService>(
        &self,
        ferry_capacity: i32,
        ferry_service: Arc<F>,
        car_service: Arc<C>,
        passenger_service: Arc<P>,
    ) -> anyhow::Result<DataSet> {
        // Step 1: Create the ferry
        let ferry = ferry_service.create(ferry_capacity).await?;

        // Step 2: Initialize variables
        let mut cars = Vec::new();
        let mut passengers = Vec::new();
        let mut remaining_capacity = ferry_capacity;

        // Step 3: Create cars and passengers for each car
        while remaining_capacity >= 6 {
            // Create a car
            let car = car_service.create().await?;
            cars.push(car.clone());

            // Create 5 passengers for the car
            for _ in 0..5 {
                let passenger = passenger_service
                    .create(Some(car.id.clone()), ferry.id.clone())
                    .await?;
                passengers.push(passenger);
            }

            // Reduce the remaining capacity
            remaining_capacity -= 6;
        }

        // Step 4: Create passengers without cars to fill the remaining spots
        for _ in 0..remaining_capacity {
            let passenger = passenger_service.create(None, ferry.id.clone()).await?;
            passengers.push(passenger);
        }

        Ok(DataSet {
            ferry,
            cars,
            passengers,
        })
    }
}
