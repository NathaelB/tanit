#[derive(Debug, serde::Serialize)]
pub struct Passenger {
    pub id: String,
    pub car_id: Option<String>,
    pub ferry_id: String,
    pub firstname: String,
    pub lastname: String,
    pub sex: bool,
}
