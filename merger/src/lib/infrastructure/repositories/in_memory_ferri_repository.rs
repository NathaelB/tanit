use std::sync::Arc;

use futures::lock::Mutex;
use tracing::info;

use crate::domain::ferri::{models::Ferri, ports::FerriRepository};

#[derive(Clone, Default)]
pub struct InMemoryFerriRepository {
    ferries: Arc<Mutex<Vec<Ferri>>>,
}

impl InMemoryFerriRepository {
    pub fn new() -> Self {
        Self {
            ferries: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl FerriRepository for InMemoryFerriRepository {
    async fn save(&self, ferri: Ferri) -> anyhow::Result<()> {
        let mut ferries = self.ferries.lock().await;
        ferries.push(ferri.clone());

        info!("Saved ferry: {:?}", ferri);

        Ok(())
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Ferri>> {
        let ferries = self.ferries.lock().await;

        Ok(ferries.clone())
    }

    async fn find_by_id(&self, id: &str) -> anyhow::Result<Option<Ferri>> {
        let ferries = self.ferries.lock().await;

        let ferry = ferries.iter().find(|f| f.id == id);

        Ok(ferry.cloned())
    }

    async fn delete_by_id(&self, id: &str) -> anyhow::Result<()> {
        let mut ferries = self.ferries.lock().await;
        ferries.retain(|f| f.id != id);

        Ok(())
    }
}
