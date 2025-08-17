use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Args {
    Value(String),
    Map(HashMap<String, Args>),
    List(Vec<Args>),
}