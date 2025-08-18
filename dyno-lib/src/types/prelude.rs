use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeMeta {
    pub kind: String,
    pub api_version: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMeta {
    pub name: Option<String>,
    pub gn: Option<String>,
    pub namespace: String,
    pub uid: String,
    pub creation_timestamp: DateTime<Utc>,
    pub deletion_timestamp: DateTime<Utc>,
    pub tags: Vec<String>,
    pub labels: HashMap<String, String>
}
