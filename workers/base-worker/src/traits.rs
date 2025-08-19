use crate::types::{Task, TaskResult, WorkerError};

pub trait TaskRunner {
    fn parse_task(&self, raw: &str) -> Result<Task, WorkerError>;
    fn start_task(&self, task: &Task) -> Result<TaskResult, WorkerError>;
}
