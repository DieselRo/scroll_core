//! Trait and utilities for persisting ScrollSessions.
//! Implementations exist for both in-memory and database-backed storage.
//! See [Sessions](../../AGENTS.md#contextframeengine) for usage.
// sessions/session_service.rs
// =======================================================
// Defines the SessionService trait and core session ops.
// Mirrors ADK BaseSessionService pattern.
// =======================================================

use crate::sessions::session::ScrollSession;
use async_trait::async_trait;
use std::collections::HashMap;

use crate::events::scroll_event::ScrollEvent;

use std::error::Error;

/// Request filter for session retrieval.
pub struct GetSessionConfig {
    pub num_recent_events: Option<usize>,
    pub after_timestamp: Option<f64>,
}

/// Sessions list response.
pub struct ListSessionsResponse {
    pub sessions: Vec<ScrollSession>,
}

/// Events list response.
pub struct ListEventsResponse {
    pub events: Vec<ScrollEvent>,
    pub next_page_token: Option<String>,
}

/// Trait for session backends (in-memory, database, etc).
#[async_trait]
pub trait SessionService {
    async fn create_session(
        &self,
        app_name: &str,
        user_id: &str,
        state: Option<HashMap<String, String>>,
        session_id: Option<String>,
    ) -> Result<ScrollSession, Box<dyn Error>>;

    async fn get_session(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
        config: Option<GetSessionConfig>,
    ) -> Result<Option<ScrollSession>, Box<dyn Error>>;

    async fn list_sessions(
        &self,
        app_name: &str,
        user_id: &str,
    ) -> Result<ListSessionsResponse, Box<dyn Error>>;

    async fn delete_session(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
        config: Option<GetSessionConfig>,
    ) -> Result<(), Box<dyn Error>>;

    async fn list_events(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
    ) -> Result<ListEventsResponse, Box<dyn Error>>;

    async fn append_event(
        &self,
        session: &mut ScrollSession,
        event: ScrollEvent,
    ) -> Result<ScrollEvent, Box<dyn Error>>;

    async fn close_session(&self, session: &mut ScrollSession) -> Result<(), Box<dyn Error>>;
}
