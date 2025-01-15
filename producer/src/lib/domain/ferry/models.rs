#[derive(Debug, serde::Serialize)]
pub struct Ferry {
    pub id: String,
    pub name: String,
    pub capacity: i32,
}
