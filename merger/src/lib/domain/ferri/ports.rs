use std::future::Future;

use crate::domain::passenger::models::Passenger;

use super::models::Ferri;

pub trait FerriService: Clone + Send + Sync + 'static {
    fn add_ferry(&self, ferri: Ferri) -> impl Future<Output = anyhow::Result<()>> + Send;
    fn find_all(&self) -> impl Future<Output = anyhow::Result<Vec<Ferri>>> + Send;
    fn find_by_id(&self, id: &str) -> impl Future<Output = anyhow::Result<Option<Ferri>>> + Send;
    fn send_to_saver(
        &self,
        passengers: Vec<Passenger>,
        ferry: Ferri,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
    fn delete_by_id(&self, id: &str) -> impl Future<Output = anyhow::Result<()>> + Send;
}

pub trait FerriRepository: Clone + Send + Sync + 'static {
    fn save(&self, ferri: Ferri) -> impl Future<Output = anyhow::Result<()>> + Send;
    fn find_all(&self) -> impl Future<Output = anyhow::Result<Vec<Ferri>>> + Send;
    fn find_by_id(&self, id: &str) -> impl Future<Output = anyhow::Result<Option<Ferri>>> + Send;
    fn delete_by_id(&self, id: &str) -> impl Future<Output = anyhow::Result<()>> + Send;
}
