// ==================================
// src/adk/sessions/state.rs
// ==================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// State management for user, app, and session states
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SessionState {
    /// User-specific state
    #[serde(default)]
    pub user_state: HashMap<String, serde_json::Value>,
    
    /// App-specific state
    #[serde(default)]
    pub app_state: HashMap<String, serde_json::Value>,
    
    /// Session-specific state
    #[serde(default)]
    pub session_state: HashMap<String, serde_json::Value>,
}

impl SessionState {
    /// Create a new session state
    pub fn new() -> Self {
        Self {
            user_state: HashMap::new(),
            app_state: HashMap::new(),
            session_state: HashMap::new(),
        }
    }
    
    /// Get a value from user state
    pub fn get_user_state(&self, key: &str) -> Option<&serde_json::Value> {
        self.user_state.get(key)
    }
    
    /// Set a value in user state
    pub fn set_user_state(&mut self, key: String, value: serde_json::Value) {
        self.user_state.insert(key, value);
    }
    
    /// Get a value from app state
    pub fn get_app_state(&self, key: &str) -> Option<&serde_json::Value> {
        self.app_state.get(key)
    }
    
    /// Set a value in app state
    pub fn set_app_state(&mut self, key: String, value: serde_json::Value) {
        self.app_state.insert(key, value);
    }
    
    /// Get a value from session state
    pub fn get_session_state(&self, key: &str) -> Option<&serde_json::Value> {
        self.session_state.get(key)
    }
    
    /// Set a value in session state
    pub fn set_session_state(&mut self, key: String, value: serde_json::Value) {
        self.session_state.insert(key, value);
    }
}

/// Configuration for retrieving a session
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSessionConfig {
    /// Whether to include events in the response
    #[serde(default)]
    pub include_events: bool,
    
    /// Maximum number of events to include (from most recent)
    pub max_events: Option<u32>,
}