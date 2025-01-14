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
}
