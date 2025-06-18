// ==================================
// src/adk/sessions/base_session_service.rs
// ==================================

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use crate::adk::common::error::AdkError;
use crate::adk::events::event::Event;
use crate::adk::sessions::session::{ListSessionsResponse, Session};
use crate::adk::sessions::state::GetSessionConfig;

/// Base trait for session services
#[async_trait]
pub trait BaseSessionService: Send + Sync {
    /// Create a new session
    async fn create_session(
        &self,
        app_name: &str,
        user_id: &str,
        state: Option<HashMap<String, serde_json::Value>>,
        session_id: Option<&str>,
    ) -> Result<Session, AdkError>;
    
    /// Get a session by ID
    async fn get_session(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
        config: Option<GetSessionConfig>,
    ) -> Result<Option<Session>, AdkError>;
    
    /// List sessions for a user
    async fn list_sessions(
        &self,
        app_name: &str,
        user_id: &str,
    ) -> Result<ListSessionsResponse, AdkError>;
    
    /// Delete a session
    async fn delete_session(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
    ) -> Result<(), AdkError>;
    
    /// Add an event to a session
    async fn append_event(
        &self,
        session: &mut Session,
        event: Event,
    ) -> Result<Event, AdkError>;
    
    /// Update session state
    async fn update_session_state(
        &self,
        session: &mut Session,
        state: HashMap<String, serde_json::Value>,
    ) -> Result<(), AdkError>;
    
    /// Close a session
    async fn close_session(
        &self,
        session: &mut Session,
    ) -> Result<(), AdkError>;
}