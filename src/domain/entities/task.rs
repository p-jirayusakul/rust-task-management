use chrono::NaiveDateTime;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub task_status_id: Option<i64>,
    pub priority_levels_id: Option<i64>,
    pub created_by: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TaskID {
    pub id: i64,
}

pub struct TaskCreateEntity {
    pub title: String,
    pub description: Option<String>,
    pub task_status_id: i64,
    pub priority_levels_id: i64,
    pub created_by: i64,
}

pub struct UpdateTask {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub task_status_id: i64,
    pub priority_levels_id: i64,
    pub updated_by: i64,
}

pub struct UpdateTaskStatus {
    pub id: i64,
    pub task_status_id: i64,
    pub updated_by: i64,
}

pub struct UpdateTaskPriorityLevels {
    pub id: i64,
    pub priority_levels_id: i64,
    pub updated_by: i64,
}