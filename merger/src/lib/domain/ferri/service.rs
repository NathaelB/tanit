use std::future::Future;

use super::{
    models::Ferri,
    ports::{FerriRepository, FerriService},
};

#[derive(Clone)]
pub struct FerriServiceImpl<F>
where
    F: FerriRepository,
{
    pub ferri_repository: F,
}

impl<F> FerriServiceImpl<F>
where
    F: FerriRepository,
{
    pub fn new(ferri_repository: F) -> Self {
        Self { ferri_repository }
    }
}

impl<F> FerriService for FerriServiceImpl<F>
where
    F: FerriRepository,
{
    async fn add_ferry(&self, ferri: Ferri) -> anyhow::Result<()> {
        self.ferri_repository.save(ferri).await
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Ferri>> {
        self.ferri_repository.find_all().await
    }

    async fn find_by_id(&self, id: &str) -> anyhow::Result<Option<Ferri>> {
        self.ferri_repository.find_by_id(id).await
    }
}
