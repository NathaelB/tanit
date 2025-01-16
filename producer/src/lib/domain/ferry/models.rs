#[derive(Debug, serde::Serialize, Clone)]
pub struct Ferry {
    pub id: String,
    pub name: String,
    pub capacity: i32,
}
