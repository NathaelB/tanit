use super::{models::Passenger, ports::PassengerService};
use fake::faker::name::fr_fr::{FirstName, LastName};
use fake::Fake;
use rand::Rng;

#[derive(Clone, Default)]
pub struct PassengerServiceImpl;

impl PassengerService for PassengerServiceImpl {
    async fn create(&self, car_id: Option<String>, ferry_id: String) -> anyhow::Result<Passenger> {
        let mut rng = rand::thread_rng();
        let passenger = Passenger {
            id: uuid::Uuid::new_v4().to_string(),
            car_id,
            ferry_id,
            firstname: FirstName().fake(),
            lastname: LastName().fake(),
            sex: rng.gen_bool(0.5),
        };

        Ok(passenger)
    }
}
