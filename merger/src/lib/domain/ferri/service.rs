use std::sync::Arc;

use tracing::info;

use crate::{
    application::ports::MessagingPort,
    domain::{car::ports::CarService, passenger::models::Passenger},
};

use super::{
    models::Ferri,
    ports::{FerriRepository, FerriService},
};

#[derive(Clone)]
pub struct FerriServiceImpl<F, C, M>
where
    F: FerriRepository,
    C: CarService,
    M: MessagingPort,
{
    pub ferri_repository: F,
    pub car_service: Arc<C>,
    pub messaging: Arc<M>,
}

impl<F, C, M> FerriServiceImpl<F, C, M>
where
    F: FerriRepository,
    C: CarService,
    M: MessagingPort,
{
    pub fn new(ferri_repository: F, car_service: Arc<C>, messaging: Arc<M>) -> Self {
        Self {
            ferri_repository,
            car_service: Arc::clone(&car_service),
            messaging: Arc::clone(&messaging),
        }
    }
}

impl<F, C, M> FerriService for FerriServiceImpl<F, C, M>
where
    F: FerriRepository,
    C: CarService,
    M: MessagingPort,
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

    async fn send_to_saver(&self, passengers: Vec<Passenger>, ferry: Ferri) -> anyhow::Result<()> {
        info!("Sending passengers to saver");

        // je veux récupérer tout les car_id qui ne sont pas Option pour avoir un Vec<String>
        let car_ids: Vec<String> = passengers.iter().filter_map(|p| p.car_id.clone()).collect();

        let cars = self.car_service.find_from_ids(car_ids.clone()).await?;

        let payload = ferry.merge(cars, passengers);

        // Publish to a topic kafka
        self.messaging
            .publish_message(String::from("saver"), payload)
            .await?;

        self.car_service.delete_by_ids(car_ids).await?;

        self.ferri_repository.delete_by_id(&ferry.id).await?;
        // supprimer les passagers de la base de données
        Ok(())
    }

    async fn delete_by_id(&self, id: &str) -> anyhow::Result<()> {
        self.ferri_repository.delete_by_id(id).await
    }
}
