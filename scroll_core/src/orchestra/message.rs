// src/orchestra/message.rs

use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AgentMessage {
    pub id: Uuid,
    pub from: String,
    pub to: String, // "validator" or "broadcast"
    pub payload: Value,
    pub trace: Vec<String>, // lineage of hands
}
