use super::{models::Car, ports::CarService};

use fake::faker::automotive::fr_fr::LicencePlate;
use fake::faker::lorem::fr_fr::Word;

use fake::Fake;

#[derive(Clone, Default)]
pub struct CarServiceImpl;

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
