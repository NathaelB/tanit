use fake::faker::automotive::fr_fr::LicencePlate;
use fake::{faker::company::fr_fr::CompanyName, Fake};
use fake::faker::lorem::fr_fr::Word;
use fake::faker::name::fr_fr::{FirstName, LastName};
use rand::Rng;

use super::{models::Car, ports::CarService};
use super::{models::Ferry, ports::FerryService};
use super::{models::Passenger, ports::PassengerService};

#[derive(Clone, Default)]
pub struct FerryServiceImpl;

#[derive(Clone, Default)]
pub struct CarServiceImpl;

#[derive(Clone, Default)]
pub struct PassengerServiceImpl;

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