#[derive(Debug)]
pub struct Ferri {
    pub id: String,
    pub name: String,
    pub capacity: u8,
}

#[derive(Debug)]
pub struct Car {
    pub id: String,
    pub licence_plate: String,
    pub brand: String,
    pub color: String,
    pub capacity: u8,
}

#[derive(Debug)]
pub struct Passenger {
    pub id: String,
    pub car_id: Option<String>,
    pub ferri_id: String,
    pub firstname: String,
    pub lastname: String,
    pub sex: bool,
}