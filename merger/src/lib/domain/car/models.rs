use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Car {
  pub id: String,
  pub license_plate: String,
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