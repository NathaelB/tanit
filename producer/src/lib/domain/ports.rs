use super::models::DataSet;
use anyhow::Result;
use std::future::Future;

pub trait DataSetService: Clone + Send + Sync + 'static {
    fn create_data_set(&self, capacity_ferry: i32) -> impl Future<Output = Result<DataSet>> + Send;
}
