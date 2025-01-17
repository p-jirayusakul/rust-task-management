use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MasterDataTaskStatus {
    pub id: i64,
    pub title: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MasterDataRole {
    pub id: i64,
    pub title: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MasterDataPriorityLevels {
    pub id: i64,
    pub title: String,
    pub code: String,
}