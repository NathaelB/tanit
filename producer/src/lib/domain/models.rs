use super::{car::models::Car, ferry::models::Ferry, passenger::models::Passenger};

#[derive(Debug, serde::Serialize)]
pub struct DataSet {
    pub(crate) ferry: Ferry,
    pub(crate) cars: Vec<Car>,
    pub(crate) passengers: Vec<Passenger>,
}
