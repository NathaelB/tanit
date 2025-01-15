use std::sync::Arc;

use fake::faker::automotive::fr_fr::LicencePlate;
use fake::faker::lorem::fr_fr::Word;
use fake::faker::name::fr_fr::{FirstName, LastName};
use fake::{faker::company::fr_fr::CompanyName, Fake};
use rand::Rng;

use super::{models::Car, ports::CarService};
use super::{models::DataSet, ports::DataSetService};
use super::{models::Ferry, ports::FerryService};
use super::{models::Passenger, ports::PassengerService};

#[derive(Clone, Default)]
pub struct FerryServiceImpl;

#[derive(Clone, Default)]
pub struct CarServiceImpl;

#[derive(Clone, Default)]
pub struct PassengerServiceImpl;

#[derive(Clone, Default)]
pub struct DataSetServiceImpl;

impl FerryService for FerryServiceImpl {
    async fn create(&self, capacity: i32) -> anyhow::Result<Ferry> {
        let ferry = Ferry {
            id: uuid::Uuid::new_v4().to_string(),
            name: CompanyName().fake(),
            capacity,
        };

        Ok(ferry)
    }
}

impl CarService for CarServiceImpl {
    async fn create(&self) -> anyhow::Result<Car> {
        let car = Car {
            id: uuid::Uuid::new_v4().to_string(),
            licence_plate: LicencePlate().fake(),
            brand: Word().fake(),
            color: Word().fake(),
            capacity: 5,
        };

        Ok(car)
    }
}

impl PassengerService for PassengerServiceImpl {
    async fn create(&self, car_id: Option<String>, ferry_id: String) -> anyhow::Result<Passenger> {
        let mut rng = rand::thread_rng();
        let passenger = Passenger {
            id: uuid::Uuid::new_v4().to_string(),
            car_id,
            ferry_id,
            firstname: FirstName().fake(),
            lastname: LastName().fake(),
            sex: rng.gen_bool(0.5),
        };

        Ok(passenger)
    }
}

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
