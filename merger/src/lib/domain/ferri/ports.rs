use std::future::Future;

use super::models::Ferri;

pub trait FerriService: Clone + Send + Sync + 'static {
    fn add_ferry(&self, ferri: Ferri) -> impl Future<Output = anyhow::Result<()>> + Send;
}

pub trait FerriRepository: Clone + Send + Sync + 'static {
    fn save(&self, ferri: Ferri) -> impl Future<Output = anyhow::Result<()>> + Send;
}
