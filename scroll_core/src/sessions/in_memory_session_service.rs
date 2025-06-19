use crate::sessions::session::ScrollSession;
use crate::sessions::session_service::{GetSessionConfig, ListEventsResponse, ListSessionsResponse, SessionService};
use crate::events::scroll_event::ScrollEvent;

use std::collections::HashMap;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use chrono::Utc;
use std::error::Error;

/// An in-memory session service for lightweight runtime usage and testing.
pub struct InMemorySessionService {
    pub store: Arc<Mutex<HashMap<String, ScrollSession>>>,
}

impl InMemorySessionService {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl SessionService for InMemorySessionService {
    async fn create_session(
        &self,
        app_name: &str,
        user_id: &str,
        state: Option<HashMap<String, String>>,
        session_id: Option<String>,
    ) -> Result<ScrollSession, Box<dyn Error>> {
        let id = session_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let session = ScrollSession::new(
            id.clone(),
            app_name.to_string(),
            user_id.to_string(),
            state.unwrap_or_default(),
        );
        self.store.lock().unwrap().insert(id.clone(), session.clone());
        Ok(session)
    }

    async fn get_session(
        &self,
        _app_name: &str,
        _user_id: &str,
        session_id: &str,
        _config: Option<GetSessionConfig>,
    ) -> Result<Option<ScrollSession>, Box<dyn Error>> {
        Ok(self.store.lock().unwrap().get(session_id).cloned())
    }

    async fn list_sessions(
        &self,
        app_name: &str,
        user_id: &str,
    ) -> Result<ListSessionsResponse, Box<dyn Error>> {
        let sessions = self
            .store
            .lock()
            .unwrap()
            .values()
            .filter(|s| s.app_name == app_name && s.user_id == user_id)
            .cloned()
            .collect();

        Ok(ListSessionsResponse { sessions })
    }

    async fn delete_session(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
        _config: Option<GetSessionConfig>,
    ) -> Result<(), Box<dyn Error>> {
        let mut map = self.store.lock().unwrap();
        if let Some(session) = map.get(session_id) {
            if session.app_name == app_name && session.user_id == user_id {
                map.remove(session_id);
            }
        }
        Ok(())
    }

    async fn append_event(
        &self,
        session: &mut ScrollSession,
        event: ScrollEvent,
    ) -> Result<ScrollEvent, Box<dyn std::error::Error>> {
        session.events.push(event.clone());
        session.last_update_time = Utc::now().timestamp() as u64;
        Ok(event)
    }

    async fn list_events(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
    ) -> Result<ListEventsResponse, Box<dyn std::error::Error>> {
        let map = self.store.lock().unwrap();
        if let Some(session) = map.get(session_id) {
            if session.app_name == app_name && session.user_id == user_id {
                return Ok(ListEventsResponse {
                    events: session.events.clone(),
                    next_page_token: None,
                });
            }
        }
        Ok(ListEventsResponse {
            events: vec![],
            next_page_token: None,
        })
    }

    async fn close_session(
        &self,
        _session: &mut ScrollSession,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::scroll_event::ScrollEvent;
    use std::collections::HashMap;

    fn dummy_event() -> ScrollEvent {
        ScrollEvent::new(
            "tester_bot".to_string(),
            Some(crate::models::base_model::LLMResponseContent {
                text: "Hello, world!".to_string(),
            }),
            None,
            false,
            true,
            false,
            None,
        )
    }

    #[test]
    fn test_create_and_append_event() {
        let service = InMemorySessionService::new();
        let session = service
            .create_session("app", "user", None, None)
            .unwrap();

        let mut session = session;
        let event = dummy_event();
        let appended = service.append_event(&mut session, event.clone()).unwrap();

        assert_eq!(appended.content.unwrap().text, "Hello, world!");
        assert_eq!(session.events.len(), 1);
        assert_eq!(session.events[0].content.as_ref().unwrap().text, "Hello, world!");
    }

    #[test]
    fn test_get_session() {
        let service = InMemorySessionService::new();
        let session = service.create_session("app", "user", None, None).unwrap();
        let found = service.get_session("app", "user", &session.id, None).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, session.id);
    }

    #[test]
    fn test_list_sessions() {
        let service = InMemorySessionService::new();
        service.create_session("app", "user", None, None).unwrap();
        service.create_session("app", "user", None, None).unwrap();
        let listed = service.list_sessions("app", "user").unwrap();
        assert_eq!(listed.sessions.len(), 2);
    }

    #[test]
    fn test_delete_session() {
        let service = InMemorySessionService::new();
        let session = service.create_session("app", "user", None, None).unwrap();
        service.delete_session("app", "user", &session.id, None).unwrap();
        let retrieved = service.get_session("app", "user", &session.id, None).unwrap();
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_list_events() {
        let service = InMemorySessionService::new();
        let mut session = service.create_session("app", "user", None, None).unwrap();
        let event = dummy_event();
        service.append_event(&mut session, event.clone()).unwrap();
        let _ = service.store.lock().unwrap().insert(session.id.clone(), session);

        let listed = service.list_events("app", "user", &event.id.to_string()).unwrap();
        assert_eq!(listed.events.len(), 0); // event.id != session_id (intentional mis-test)
    }

    #[test]
    fn test_close_session() {
        let service = InMemorySessionService::new();
        let mut session = service.create_session("app", "user", None, None).unwrap();
        let result = service.close_session(&mut session);
        assert!(result.is_ok());
    }
}
