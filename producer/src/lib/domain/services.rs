use crate::application::ports::MessagingPort;

use super::car::ports::CarService;
use std::sync::Arc;

use super::ferry::ports::FerryService;

use super::passenger::ports::PassengerService;
use super::{models::DataSet, ports::DataSetService};

#[derive(Clone, Default)]
pub struct DataSetServiceImpl<F, C, P, M>
where
    F: FerryService,
    C: CarService,
    P: PassengerService,
    M: MessagingPort,
{
    ferry_service: Arc<F>,
    car_service: Arc<C>,
    passenger_service: Arc<P>,
    messaging_service: Arc<M>,
}

impl<F, C, P, M> DataSetServiceImpl<F, C, P, M>
where
    F: FerryService,
    C: CarService,
    P: PassengerService,
    M: MessagingPort,
{
    pub fn new(
        ferry_service: Arc<F>,
        car_service: Arc<C>,
        passenger_service: Arc<P>,
        messaging_service: Arc<M>,
    ) -> Self {
        Self {
            ferry_service,
            car_service,
            passenger_service,
            messaging_service,
        }
    }
}

impl<F, C, P, M> DataSetService for DataSetServiceImpl<F, C, P, M>
where
    F: FerryService,
    C: CarService,
    P: PassengerService,
    M: MessagingPort,
{
    async fn create_ferry(&self, ferry_capacity: i32) -> anyhow::Result<DataSet> {
        // Step 1: Create the ferry
        let ferry = self.ferry_service.create(ferry_capacity).await?;

        self.messaging_service
            .publish_message(String::from("ferries"), ferry.clone())
            .await?;

        // Step 2: Initialize variables
        let mut cars = Vec::new();
        let mut passengers = Vec::new();
        let mut remaining_capacity = ferry_capacity;

        // Step 3: Create cars and passengers for each car
        while remaining_capacity >= 10 {
            // Create a car
            let car = self.car_service.create().await?;
            self.messaging_service
                .publish_message(String::from("cars"), car.clone())
                .await?;
            cars.push(car.clone());

            // Create 5 passengers for the car
            for _ in 0..5 {
                let passenger = self
                    .passenger_service
                    .create(Some(car.id.clone()), ferry.id.clone())
                    .await?;
                self.messaging_service
                    .publish_message(String::from("passengers"), passenger.clone())
                    .await?;
                passengers.push(passenger);
            }

            // Reduce the remaining capacity
            remaining_capacity -= 5;
        }

        // Step 4: Create passengers without cars to fill the remaining spots
        for _ in 0..remaining_capacity {
            let passenger = self
                .passenger_service
                .create(None, ferry.id.clone())
                .await?;

            self.messaging_service
                .publish_message(String::from("passengers"), passenger.clone())
                .await?;
            passengers.push(passenger);
        }

        Ok(DataSet {
            ferry,
            cars,
            passengers,
        })
    }
}
