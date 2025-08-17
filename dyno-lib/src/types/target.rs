use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub name: String,
    pub namsepace: String,
    pub identifier: String,
    pub transport: TransportMode,
    pub labels: HashMap<String,String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransportMode {
    SSH,
    WINRM,
    HTTP2,
    GRPC,
    DOCKER,
    KUBERNETES,
    AWS
}