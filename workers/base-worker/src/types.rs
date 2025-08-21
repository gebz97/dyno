use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub steps: Vec<Step>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    pub id: String,
    pub name: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: String,
    pub step_results: Vec<StepResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StepResult {
    pub step_id: String,
    pub status: StepStatus,
    pub output: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StepStatus {
    Succeeded,
    Failed(String),
    Skipped(String),
    Ignored(String),
    Retried(u32, String)
}

#[derive(Debug, thiserror::Error)]
pub enum WorkerError {
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Execution error: {0}")]
    ExecutionError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
}