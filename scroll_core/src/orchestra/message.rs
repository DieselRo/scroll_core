//! Defines the AgentMessage type used for communication on the bus.
//! Messages carry payloads and track their lineage for debugging.
//! See [Orchestra](../AGENTS.md#invocationmanager) for how messages flow.
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
