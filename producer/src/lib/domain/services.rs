use fake::{faker::company::fr_fr::CompanyName, Fake};

use super::{models::Ferry, ports::FerryService};

#[derive(Clone, Default)]
pub struct FerryServiceImpl;

impl FerryService for FerryServiceImpl {
  async fn create(&self, capacity: i32) -> anyhow::Result<Ferry> {
      let ferry = Ferry {
        id: uuid::Uuid::new_v4().to_string(),
        name: CompanyName().fake(),
        capacity,
      };

      Ok(ferry)
  }
}