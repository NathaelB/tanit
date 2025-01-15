use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Passenger {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreatePassengerEvent {
    pub id: String,
    pub name: String,
}

impl Passenger {
    pub fn from_event(event: CreatePassengerEvent) -> Self {
        Passenger {
            id: event.id,
            name: event.name,
        }
    }
}