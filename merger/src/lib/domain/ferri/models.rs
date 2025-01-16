use serde::{Deserialize, Serialize};

use crate::domain::{car::models::Car, passenger::models::Passenger};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ferri {
    pub id: String,
    pub name: String,
    pub capacity: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct FerriPayloadSaver {
    pub id: String,
    pub name: String,
    pub capacity: i32,
    pub passengers: Vec<Passenger>,
    pub cars: Vec<Car>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateFerryEvent {
    pub id: String,
    pub name: String,
    pub capacity: i32,
}

impl Ferri {
    pub fn from_event(event: CreateFerryEvent) -> Self {
        Ferri {
            id: event.id,
            name: event.name,
            capacity: event.capacity,
        }
    }

    pub fn merge(&self, cars: Vec<Car>, passengers: Vec<Passenger>) -> FerriPayloadSaver {
        FerriPayloadSaver {
            id: self.id.clone(),
            name: self.name.clone(),
            capacity: self.capacity,
            passengers,
            cars,
        }
    }
}
