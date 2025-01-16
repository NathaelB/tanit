use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Passenger {
    pub id: String,
    pub car_id: Option<String>,
    pub ferry_id: String,
    pub firstname: String,
    pub lastname: String,
    pub sex: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreatePassengerEvent {
    pub id: String,
    pub car_id: Option<String>,
    pub ferry_id: String,
    pub firstname: String,
    pub lastname: String,
    pub sex: bool,
}

impl Passenger {
    pub fn from_event(event: CreatePassengerEvent) -> Self {
        Passenger {
            id: event.id,
            car_id: event.car_id,
            ferry_id: event.ferry_id,
            firstname: event.firstname,
            lastname: event.lastname,
            sex: event.sex,
        }
    }
}
