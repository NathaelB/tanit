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
    fn add_ferry(&self, ferri: Ferri) -> impl Future<Output = anyhow::Result<()>> + Send {
        self.ferri_repository.save(ferri)
    }
}
