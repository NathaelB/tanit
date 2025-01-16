use std::future::Future;

use super::models::Ferri;

pub trait FerriService: Clone + Send + Sync + 'static {
    fn add_ferry(&self, ferri: Ferri) -> impl Future<Output = anyhow::Result<()>> + Send;
    fn find_all(&self) -> impl Future<Output = anyhow::Result<Vec<Ferri>>> + Send;
    fn find_by_id(&self, id: &str) -> impl Future<Output = anyhow::Result<Option<Ferri>>> + Send;
}

pub trait FerriRepository: Clone + Send + Sync + 'static {
    fn save(&self, ferri: Ferri) -> impl Future<Output = anyhow::Result<()>> + Send;
    fn find_all(&self) -> impl Future<Output = anyhow::Result<Vec<Ferri>>> + Send;
    fn find_by_id(&self, id: &str) -> impl Future<Output = anyhow::Result<Option<Ferri>>> + Send;
}
