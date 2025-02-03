use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TaskRequest {
    #[validate(length(min = 1))]
    pub title: String,

    pub description: Option<String>,

    #[serde(rename = "taskStatusId")]
    pub task_status_id: i64,

    #[serde(rename = "priorityLevelsId")]
    pub priority_levels_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateTaskStatusRequest {
    #[serde(rename = "taskStatusId")]
    pub task_status_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateTaskPriorityLevelsRequest {
    #[serde(rename = "priorityLevelsId")]
    pub priority_levels_id: i64,
}