// ==================================
// src/adk/sessions/in_memory_service.rs
// ==================================

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::adk::common::error::AdkError;
use crate::adk::events::event::Event;
use crate::adk::sessions::base_session_service::BaseSessionService;
use crate::adk::sessions::session::{ListSessionsResponse, Session, SessionSummary};
use crate::adk::sessions::state::GetSessionConfig;

/// In-memory implementation of session service
pub struct InMemorySessionService {
    // Sessions stored by app_name -> user_id -> session_id
    sessions: Arc<Mutex<HashMap<String, HashMap<String, HashMap<String, Session>>>>>,
}

impl InMemorySessionService {
    /// Create a new in-memory session service
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl BaseSessionService for InMemorySessionService {
    async fn create_session(
        &self,
        app_name: &str,
        user_id: &str,
        state: Option<HashMap<String, serde_json::Value>>,
        session_id: Option<&str>,
    ) -> Result<Session, AdkError> {
        let id = session_id
            .map(|s| s.to_string())
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        let session = Session::new(id, app_name.to_string(), user_id.to_string(), state);

        let mut sessions = self
            .sessions
            .lock()
            .map_err(|e| AdkError::Other(e.to_string()))?;

        // Ensure app entry exists
        if !sessions.contains_key(app_name) {
            sessions.insert(app_name.to_string(), HashMap::new());
        }

        // Ensure user entry exists
        let app_sessions = sessions.get_mut(app_name).unwrap();
        if !app_sessions.contains_key(user_id) {
            app_sessions.insert(user_id.to_string(), HashMap::new());
        }

        // Store session
        let user_sessions = app_sessions.get_mut(user_id).unwrap();
        let session_id = session.id.clone();
        user_sessions.insert(session_id.clone(), session.clone());

        Ok(session)
    }

    async fn get_session(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
        config: Option<GetSessionConfig>,
    ) -> Result<Option<Session>, AdkError> {
        let sessions = self
            .sessions
            .lock()
            .map_err(|e| AdkError::Other(e.to_string()))?;

        // Retrieve session
        let session = sessions
            .get(app_name)
            .and_then(|app_sessions| app_sessions.get(user_id))
            .and_then(|user_sessions| user_sessions.get(session_id))
            .cloned();

        // Apply config filter if provided
        if let Some(mut session) = session {
            if let Some(config) = config {
                if !config.include_events {
                    session.events.clear();
                } else if let Some(max_events) = config.max_events {
                    if session.events.len() > max_events as usize {
                        let start = session.events.len() - max_events as usize;
                        session.events = session.events[start..].to_vec();
                    }
                }
            }
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    async fn list_sessions(
        &self,
        app_name: &str,
        user_id: &str,
    ) -> Result<ListSessionsResponse, AdkError> {
        let sessions = self
            .sessions
            .lock()
            .map_err(|e| AdkError::Other(e.to_string()))?;

        let summaries = sessions
            .get(app_name)
            .and_then(|app_sessions| app_sessions.get(user_id))
            .map(|user_sessions| {
                user_sessions
                    .values()
                    .map(|session| SessionSummary {
                        id: session.id.clone(),
                        app_name: session.app_name.clone(),
                        user_id: session.user_id.clone(),
                        create_time: session.create_time,
                        last_update_time: session.last_update_time,
                        status: session.status.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(ListSessionsResponse {
            sessions: summaries,
            next_page_token: None,
        })
    }

    async fn delete_session(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
    ) -> Result<(), AdkError> {
        let mut sessions = self
            .sessions
            .lock()
            .map_err(|e| AdkError::Other(e.to_string()))?;

        if let Some(app_sessions) = sessions.get_mut(app_name) {
            if let Some(user_sessions) = app_sessions.get_mut(user_id) {
                user_sessions.remove(session_id);
                return Ok(());
            }
        }

        Err(AdkError::NotFound(format!(
            "Session not found: {}",
            session_id
        )))
    }

    async fn append_event(&self, session: &mut Session, event: Event) -> Result<Event, AdkError> {
        let mut sessions = self
            .sessions
            .lock()
            .map_err(|e| AdkError::Other(e.to_string()))?;

        if let Some(app_sessions) = sessions.get_mut(&session.app_name) {
            if let Some(user_sessions) = app_sessions.get_mut(&session.user_id) {
                if let Some(stored_session) = user_sessions.get_mut(&session.id) {
                    stored_session.add_event(event.clone());
                    session.add_event(event.clone());
                    return Ok(event);
                }
            }
        }

        Err(AdkError::NotFound(format!(
            "Session not found: {}",
            session.id
        )))
    }

    async fn update_session_state(
        &self,
        session: &mut Session,
        state: HashMap<String, serde_json::Value>,
    ) -> Result<(), AdkError> {
        let mut sessions = self
            .sessions
            .lock()
            .map_err(|e| AdkError::Other(e.to_string()))?;

        if let Some(app_sessions) = sessions.get_mut(&session.app_name) {
            if let Some(user_sessions) = app_sessions.get_mut(&session.user_id) {
                if let Some(stored_session) = user_sessions.get_mut(&session.id) {
                    for (key, value) in state.iter() {
                        stored_session.set_state_value(key.clone(), value.clone());
                        session.set_state_value(key.clone(), value.clone());
                    }
                    return Ok(());
                }
            }
        }

        Err(AdkError::NotFound(format!(
            "Session not found: {}",
            session.id
        )))
    }

    async fn close_session(&self, session: &mut Session) -> Result<(), AdkError> {
        let mut sessions = self
            .sessions
            .lock()
            .map_err(|e| AdkError::Other(e.to_string()))?;

        if let Some(app_sessions) = sessions.get_mut(&session.app_name) {
            if let Some(user_sessions) = app_sessions.get_mut(&session.user_id) {
                if let Some(stored_session) = user_sessions.get_mut(&session.id) {
                    stored_session.close();
                    session.close();
                    return Ok(());
                }
            }
        }

        Err(AdkError::NotFound(format!(
            "Session not found: {}",
            session.id
        )))
    }
}
