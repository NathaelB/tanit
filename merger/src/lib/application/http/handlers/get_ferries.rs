use std::sync::Arc;

use axum::{http::StatusCode, Extension};
use serde::Serialize;

use crate::domain::ferri::{models::Ferri, ports::FerriService};

use super::{ApiError, ApiSuccess};

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetFerriesResponseData(Vec<Ferri>);

pub async fn get_ferries<F: FerriService>(
    Extension(ferry_service): Extension<Arc<F>>,
) -> Result<ApiSuccess<GetFerriesResponseData>, ApiError> {
    ferry_service
        .find_all()
        .await
        .map_err(|_| ApiError::InternalServerError("Failed to fetch ferries".to_string()))
        .map(|ferries| ApiSuccess::new(StatusCode::OK, GetFerriesResponseData(ferries)))
}
