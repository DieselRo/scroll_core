// ==================================
// src/adk/sessions/session.rs
// ==================================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::adk::events::event::Event;

/// Session status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SessionStatus {
    Active,
    Closed,
}

/// Session structure representing a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    /// Unique identifier for this session
    pub id: String,
    
    /// Application name
    pub app_name: String,
    
    /// User identifier
    pub user_id: String,
    
    /// Session state - arbitrary key-value data
    #[serde(default)]
    pub state: HashMap<String, serde_json::Value>,
    
    /// Events in this session
    #[serde(default)]
    pub events: Vec<Event>,
    
    /// Timestamp of session creation
    pub create_time: f64,
    
    /// Timestamp of last update
    pub last_update_time: f64,
    
    /// Session status
    #[serde(default = "default_session_status")]
    pub status: SessionStatus,
}

fn default_session_status() -> SessionStatus {
    SessionStatus::Active
}

impl Session {
    /// Create a new session
    pub fn new(
        id: String,
        app_name: String,
        user_id: String,
        state: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        let now = Utc::now();
        let timestamp = now.timestamp_millis() as f64 / 1000.0;
        
        Self {
            id,
            app_name,
            user_id,
            state: state.unwrap_or_default(),
            events: vec![],
            create_time: timestamp,
            last_update_time: timestamp,
            status: SessionStatus::Active,
        }
    }
    
    /// Add an event to the session
    pub fn add_event(&mut self, event: Event) {
        let now = Utc::now();
        self.last_update_time = now.timestamp_millis() as f64 / 1000.0;
        self.events.push(event);
    }
    
    /// Close the session
    pub fn close(&mut self) {
        let now = Utc::now();
        self.last_update_time = now.timestamp_millis() as f64 / 1000.0;
        self.status = SessionStatus::Closed;
    }
    
    /// Get a value from session state
    pub fn get_state_value(&self, key: &str) -> Option<&serde_json::Value> {
        self.state.get(key)
    }
    
    /// Set a value in session state
    pub fn set_state_value(&mut self, key: String, value: serde_json::Value) {
        let now = Utc::now();
        self.last_update_time = now.timestamp_millis() as f64 / 1000.0;
        self.state.insert(key, value);
    }
}

/// Response structure for listing sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSessionsResponse {
    pub sessions: Vec<SessionSummary>,
    pub next_page_token: Option<String>,
}

/// Summary of a session for listing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionSummary {
    pub id: String,
    pub app_name: String,
    pub user_id: String,
    pub create_time: f64,
    pub last_update_time: f64,
    pub status: SessionStatus,
}