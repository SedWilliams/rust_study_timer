#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TimeLog {
    pub id: u32,
    pub time_spent: [u64;3],
    pub date: String,
}
