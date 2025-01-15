use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Ferri {
    pub id: String,
    pub name: String,
    pub capacity: i32,
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
}
