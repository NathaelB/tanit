#[derive(Debug, serde::Serialize)]
pub struct Ferry {
    pub id: String,
    pub name: String,
    pub capacity: i32,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Car {
    pub id: String,
    pub licence_plate: String,
    pub brand: String,
    pub color: String,
    pub capacity: i32,
}

#[derive(Debug, serde::Serialize)]
pub struct Passenger {
    pub id: String,
    pub car_id: Option<String>,
    pub ferry_id: String,
    pub firstname: String,
    pub lastname: String,
    pub sex: bool,
}

#[derive(Debug, serde::Serialize)]
pub struct DataSet {
    pub(crate) ferry: Ferry,
    pub(crate) cars: Vec<Car>,
    pub(crate) passengers: Vec<Passenger>,
}
