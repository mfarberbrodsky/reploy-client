use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct GlobalConfig {
    pub servers: HashMap<String, ServerConfig>
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    // A string of both the address and the port, for example 1.2.3.4:3000
    pub key: String, // A 128 bit encryption key
}

#[derive(Serialize, Deserialize)]
pub struct ProjectConfig {
    pub uuid: uuid::Uuid,
    pub default_servers: Vec<String>
}
