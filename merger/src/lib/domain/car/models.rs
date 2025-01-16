use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Car {
    pub id: String,
    pub brand: String,
    pub color: String,
    pub capacity: i32,
}

#[derive(Debug, Clone, Error)]
pub enum CarError {
    #[error("Error creating car: {0}")]
    CreateError(String),
    #[error("Error car not found: {0}")]
    NotFound(String),
}

impl Car {
    pub fn from_event(event: CreateCarEvent) -> Self {
        Car {
            id: event.id,
            brand: event.brand,
            color: event.color,
            capacity: event.capacity,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateCarEvent {
    pub id: String,
    pub brand: String,
    pub color: String,
    pub capacity: i32,
}
