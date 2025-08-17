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
