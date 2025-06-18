// ==================================
// src/adk/events/event.rs
// ==================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

use crate::adk::common::types::Content;
use crate::adk::events::event_actions::EventActions;

/// Event structure representing a message or action in a session
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// Unique identifier for this event
    pub id: String,
    
    /// ID of the invocation that created this event
    pub invocation_id: String,
    
    /// Author of the event (user, agent name, or system)
    pub author: String,
    
    /// Content of the event (text, images, function calls, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
    
    /// Actions attached to this event
    #[serde(default)]
    pub actions: EventActions,
    
    /// Branch identifier for multi-branch conversations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    
    /// Timestamp of the event (seconds since epoch with millisecond precision)
    pub timestamp: f64,
    
    /// Whether this is a partial event (for streaming)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<bool>,
    
    /// Whether this event completes a turn
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_complete: Option<bool>,
    
    /// Whether this event was interrupted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interrupted: Option<bool>,
    
    /// Error code if this event represents an error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    
    /// Error message if this event represents an error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    
    /// IDs of long-running tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_running_tool_ids: Option<HashSet<String>>,
}

impl Event {
    /// Create a new event from a user
    pub fn new_user_event(invocation_id: &str, content: Content) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            invocation_id: invocation_id.to_string(),
            author: "user".to_string(),
            content: Some(content),
            actions: EventActions::default(),
            branch: None,
            timestamp: now.timestamp_millis() as f64 / 1000.0,
            partial: None,
            turn_complete: None,
            interrupted: None,
            error_code: None,
            error_message: None,
            long_running_tool_ids: None,
        }
    }
    
    /// Create a new event from an agent
    pub fn new_agent_event(
        invocation_id: &str, 
        agent_name: &str, 
        content: Content,
        partial: bool
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            invocation_id: invocation_id.to_string(),
            author: agent_name.to_string(),
            content: Some(content),
            actions: EventActions::default(),
            branch: None,
            timestamp: now.timestamp_millis() as f64 / 1000.0,
            partial: Some(partial),
            turn_complete: None,
            interrupted: None,
            error_code: None,
            error_message: None,
            long_running_tool_ids: None,
        }
    }
    
    /// Create a new error event
    pub fn new_error_event(
        invocation_id: &str,
        author: &str,
        error_code: &str,
        error_message: &str
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            invocation_id: invocation_id.to_string(),
            author: author.to_string(),
            content: None,
            actions: EventActions::default(),
            branch: None,
            timestamp: now.timestamp_millis() as f64 / 1000.0,
            partial: None,
            turn_complete: Some(true),
            interrupted: None,
            error_code: Some(error_code.to_string()),
            error_message: Some(error_message.to_string()),
            long_running_tool_ids: None,
        }
    }
}