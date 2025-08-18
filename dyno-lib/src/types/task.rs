use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::target::Target;
use crate::types::step::Step;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub namespace: String,
    pub targets: Vec<Target>,
    pub steps: Vec<Step>,
    pub children: Vec<String>,
    pub labels: HashMap<String,String>,
    pub tags: Vec<String>,
}

impl Task {
    pub fn print_stuff() {}
}