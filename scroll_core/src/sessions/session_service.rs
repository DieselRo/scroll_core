// sessions/session_service.rs
// =======================================================
// Defines the SessionService trait and core session ops.
// Mirrors ADK BaseSessionService pattern.
// =======================================================

use crate::sessions::session::ScrollSession;
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
pub trait SessionService {
    fn create_session(
        &self,
        app_name: &str,
        user_id: &str,
        state: Option<HashMap<String, String>>,
        session_id: Option<String>,
    ) -> Result<ScrollSession, Box<dyn Error>>;

    fn get_session(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
        config: Option<GetSessionConfig>,
    ) -> Result<Option<ScrollSession>, Box<dyn Error>>;

    fn list_sessions(
        &self,
        app_name: &str,
        user_id: &str,
    ) -> Result<ListSessionsResponse, Box<dyn Error>>;

    fn delete_session(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
        config: Option<GetSessionConfig>,
    ) -> Result<(), Box<dyn Error>>;

    fn list_events(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
    ) -> Result<ListEventsResponse, Box<dyn Error>>;

    fn append_event(
        &self,
        session: &mut ScrollSession,
        event: ScrollEvent,
    ) -> Result<ScrollEvent, Box<dyn Error>>;

    fn close_session(
        &self,
        session: &mut ScrollSession,
    ) -> Result<(), Box<dyn Error>>;
}