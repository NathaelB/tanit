use super::models::DataSet;
use anyhow::Result;
use std::future::Future;

pub trait DataSetService: Clone + Send + Sync + 'static {
    fn create_ferry(&self, ferry_capacity: i32) -> impl Future<Output = Result<DataSet>> + Send;
}
