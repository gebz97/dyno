use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Execution {
    pub id: u32,
    pub user: u32,
    pub status: ExecutionStatus,
    pub triggered_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Triggered,
    Scheduling,
    Running,
    Succeeded,
    Retrying,
    Failed
}