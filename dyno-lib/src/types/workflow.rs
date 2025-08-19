use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::types::{execution::ExecutionStatus, task::Task};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Workflow {
    pub name: String,
    pub namespace: String,
    pub tasks: Vec<Task>,
    pub uid: String,
    pub creation_timestamp: DateTime<Utc>,
    pub deletion_timestamp: DateTime<Utc>,
    pub tags: Vec<String>,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkflowState {
    pub qualifier: String,
    pub status: ExecutionStatus,
    pub retries: u32
}

#[derive(Debug, Error)]
pub enum WorkflowError {
    #[error("Workflow has Cycles! {0}")]
    HasCycles(String),

    #[error("Duplicate Tasks! {0}")]
    DuplicateTask(String),

    #[error("Unknown Child! {0}")]
    UnknownChild(String),

    #[error("Serialization/Deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

impl Workflow {
    /// Validate workflow for duplicate tasks and cycles
    pub fn validate(&self) -> Result<(), WorkflowError> {
        let mut names = HashSet::new();
        for t in &self.tasks {
            if !names.insert(t.name.clone()) {
                return Err(WorkflowError::DuplicateTask(t.name.clone()));
            }
        }

        let task_map: HashMap<String, Task> = self
            .tasks
            .iter()
            .map(|t| (t.name.clone(), t.clone()))
            .collect();

        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();

        for t in &self.tasks {
            visit(&t.name, &task_map, &mut visiting, &mut visited)?;
        }

        Ok(())
    }
}

/// Recursive DFS to detect cycles
fn visit(
    t_name: &str,
    task_map: &HashMap<String, Task>,
    visiting: &mut HashSet<String>,
    visited: &mut HashSet<String>,
) -> Result<(), WorkflowError> {
    if visiting.contains(t_name) {
        return Err(WorkflowError::HasCycles(t_name.to_string()));
    }
    if visited.contains(t_name) {
        return Ok(());
    }

    visiting.insert(t_name.to_string());

    for child in &task_map[t_name].children {
        visit(child, task_map, visiting, visited)?;
    }

    visiting.remove(t_name);
    visited.insert(t_name.to_string());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn sample_workflow() -> Workflow {
        let ns = "myns".to_string();
        let labels = HashMap::new();
        let tags = vec!["tag1".to_string()];

        Workflow {
            name: "myworkflow".into(),
            namespace: ns.clone(),
            uid: "1234".into(),
            creation_timestamp: Utc::now(),
            deletion_timestamp: Utc::now(),
            tags: tags.clone(),
            labels: labels.clone(),
            tasks: vec![
                Task {
                    name: "task1".into(),
                    children: vec!["task2".into()],
                    steps: vec![],
                    targets: vec![],
                    namespace: ns.clone(),
                    labels: labels.clone(),
                    tags: tags.clone(),
                    qualifier: "".into(),
                },
                Task {
                    name: "task2".into(),
                    children: vec![],
                    steps: vec![],
                    targets: vec![],
                    namespace: ns,
                    labels,
                    tags,
                    qualifier: "".into(),
                },
            ],
        }
    }

    #[test]
    fn test_validate_success() {
        let wf = sample_workflow();
        assert!(wf.validate().is_ok());
    }

    #[test]
    fn test_validate_duplicate_tasks() {
        let mut wf = sample_workflow();
        // Add a duplicate
        wf.tasks.push(Task {
            name: "task1".into(),
            children: vec![],
            steps: vec![],
            targets: vec![],
            namespace: wf.namespace.clone(),
            labels: wf.labels.clone(),
            tags: wf.tags.clone(),
            qualifier: "".into(),
        });
        let res = wf.validate();
        assert!(matches!(res, Err(WorkflowError::DuplicateTask(_))));
    }

    #[test]
    fn test_validate_cycle() {
        let mut wf = sample_workflow();
        // Introduce a cycle: task2 -> task1
        wf.tasks
            .iter_mut()
            .find(|t| t.name == "task2")
            .unwrap()
            .children
            .push("task1".into());
        let res = wf.validate();
        assert!(matches!(res, Err(WorkflowError::HasCycles(_))));
    }

    #[test]
    fn test_yaml_roundtrip() {
        let wf = sample_workflow();
        let yaml = serde_yaml::to_string(&wf).unwrap();
        let wf2: Workflow = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(wf.name, wf2.name);
        assert_eq!(wf.tasks.len(), wf2.tasks.len());
    }

    #[test]
    fn test_json_roundtrip() {
        let wf = sample_workflow();
        let json = serde_json::to_string(&wf).unwrap();
        let wf2: Workflow = serde_json::from_str(&json).unwrap();
        assert_eq!(wf.name, wf2.name);
        assert_eq!(wf.tasks.len(), wf2.tasks.len());
    }
}
