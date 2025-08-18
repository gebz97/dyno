use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::types::args::Args;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub name: String,
    pub namespace: String,
    pub action: String,
    pub args: Args,
    pub labels: HashMap<String,String>,
    pub tags: Vec<String>,
}

pub struct StepSpec {
    pub qualifier: String,
    pub action: String,
    pub args: String,
    pub any_errors_fatal: bool,
    pub ignore_errors: bool,
    pub ignore_unreachable: bool,
    pub retries: u32,
    pub delay: u32,
    pub run_once: bool,
    pub delegate_to: String,
    pub target_user: String,
    pub sudo: bool,
    pub sudo_exe: String,
    pub sudo_method: String,
    pub sudo_user: String,
    pub when: Vec<String>,
    pub untill: Vec<String>,
    pub foreach: Vec<String>,
    pub foreach_var: String,
    pub transport: String,
    pub port: u16,
    pub register: String,
    pub labels: HashMap<String,String>,
    pub tags: Vec<String>,
}