use serde_derive::{Deserialize, Serialize};
use chrono::{NaiveDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct MasterDataTaskStatus {
    pub id: i64,
    pub title: String,
    pub code: String,
    pub active: bool,
    pub created_by: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MasterDataRole {
    pub id: i64,
    pub title: String,
    pub code: String,
    pub active: bool,
    pub created_by: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MasterDataPriorityLevels {
    pub id: i64,
    pub seq: i32,
    pub title: String,
    pub code: String,
    pub active: bool,
    pub created_by: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<i64>,
}