use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Theme {
    pub theme_id: i32,
    pub title: String,
    // objectives: Vec<i32>,
    pub theme_status: Status,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CreateTheme {
    pub new_title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Objective {
    pub objective_id: i32,
    pub title: String,
    // key_results: Vec<i32>,
    // initiatives: Vec<i32>,
    // projects: Vec<i32>,
    pub theme_id: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CreateObjective {
    pub new_title: String,
    pub theme_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct KeyResult {
    pub keyresult_id: i32,
    pub title: String,
    pub objective_id: i32,
    // measurements: Vec<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CreateKeyResult {
    pub new_title: String,
    pub objective_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Initiative {
    pub initiative_id: i32,
    pub title: String,
    pub objective_id: i32,
    // explanation: String,
    pub initiative_status: Status,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CreateInitiative {
    pub new_title: String,
    pub objective_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Project {
    pub project_id: i32,
    pub title: String,
    pub project_status: Status,
    pub objective_id: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CreateProject {
    pub new_title: String,
    pub objective_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Task {
    pub task_id: i32,
    pub title: String,
    pub task_status: Status,
    pub project_id: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CreateTask {
    pub new_title: String,
    pub project_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Measurement {
    pub measurement_id: i32,
    pub measurement: String,
    pub keyresult_id: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CreateMeasurement {
    pub new_title: String,
    pub keyresult_id: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "status_type")]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::NotStarted => f.write_str("Not Started"),
            Status::InProgress => f.write_str("In Progress"),
            Status::Completed => f.write_str("Completed"),
        }
    }
}
