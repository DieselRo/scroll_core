// ==================================
// src/adk/events/event_actions.rs
// ==================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Event actions that can be attached to an event
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventActions {
    /// Transfer to agent action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_to_agent: Option<TransferToAgentAction>,

    /// Custom action data
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub custom: HashMap<String, serde_json::Value>,
}

/// Action to transfer control to another agent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferToAgentAction {
    /// Name of the agent to transfer to
    pub agent_name: String,

    /// Additional context or data to pass to the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}
