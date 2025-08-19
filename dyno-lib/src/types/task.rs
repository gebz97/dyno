use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::execution::ExecutionStatus;
use crate::types::target::Target;
use crate::types::step::Step;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub namespace: String,
    pub qualifier: String,
    pub targets: Vec<Target>,
    pub steps: Vec<Step>,
    pub children: Vec<String>,
    pub labels: HashMap<String,String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskState {
    pub qualifier: String,
    pub status: ExecutionStatus,
    pub retries: u32,
    pub in_degree: u32,
}