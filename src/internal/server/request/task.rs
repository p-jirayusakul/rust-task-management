use serde::{Deserialize, Serialize};
use validator::{Validate};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTask {
    // ใช้การตรวจสอบความยาวเพื่อบังคับว่าต้องไม่ใช่ String ว่าง
    #[validate(length(min = 1))]
    pub title: String,

    pub description: Option<String>,

    #[serde(rename = "taskStatusId")]
    pub task_status_id: i64,

    #[serde(rename = "priorityLevelsId")]
    pub priority_levels_id: i64,
}