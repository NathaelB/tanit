#[derive(Clone, Debug, serde::Serialize)]
pub struct Car {
    pub id: String,
    pub brand: String,
    pub color: String,
    pub capacity: i32,
}
