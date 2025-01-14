use std::sync::Arc;

use futures::lock::Mutex;

use crate::domain::ferri::models::Ferri;

#[derive(Clone, Default)]
pub struct InMemoryFerriRepository {
    #[allow(dead_code)]
    ferries: Arc<Mutex<Vec<Ferri>>>,
}

impl InMemoryFerriRepository {
    pub fn new() -> Self {
        Self {
            ferries: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
