// ==================================
// src/adk/tests/session_tests.rs
// ==================================

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    
    use crate::adk::common::error::AdkError;
    use crate::adk::events::event::Event;
    use crate::adk::sessions::base_session_service::BaseSessionService;
    use crate::adk::sessions::in_memory_service::InMemorySessionService;
    use crate::adk::sessions::session::SessionStatus;
    use crate::adk::common::types::{Content, Part};
    
    #[tokio::test]
    async fn test_create_session() {
        let service = InMemorySessionService::new();
        
        let app_name = "test-app";
        let user_id = "test-user";
        let session_id = "test-session";
        
        let session = service.create_session(
            app_name,
            user_id,
            None,
            Some(session_id),
        ).await.unwrap();
        
        assert_eq!(session.id, session_id);
        assert_eq!(session.app_name, app_name);
        assert_eq!(session.user_id, user_id);
        assert_eq!(session.events.len(), 0);
        assert_eq!(session.status, SessionStatus::Active);
    }
    
    #[tokio::test]
    async fn test_get_session() {
        let service = InMemorySessionService::new();
        
        let app_name = "test-app";
        let user_id = "test-user";
        let session_id = "test-session";
        
        // Create a session first
        service.create_session(
            app_name,
            user_id,
            None,
            Some(session_id),
        ).await.unwrap();
        
        // Get the session
        let session = service.get_session(
            app_name,
            user_id,
            session_id,
            None,
        ).await.unwrap();
        
        assert!(session.is_some());
        let session = session.unwrap();
        assert_eq!(session.id, session_id);
    }
    
    #[tokio::test]
    async fn test_append_event() {
        let service = InMemorySessionService::new();
        
        let app_name = "test-app";
        let user_id = "test-user";
        let session_id = "test-session";
        
        // Create a session first
        let mut session = service.create_session(
            app_name,
            user_id,
            None,
            Some(session_id),
        ).await.unwrap();
        
        // Create an event
        let content = Content {
            role: Some("user".to_string()),
            parts: vec![Part {
                text: Some("Test message".to_string()),
                inline_data: None,
                function_call: None,
                function_response: None,
            }],
        };
        
        let event = Event::new_user_event("test-invocation", content);
        
        // Add the event
        let _result = service.append_event(&mut session, event).await.unwrap();
        
        // Check the event was added
        assert_eq!(session.events.len(), 1);
        assert_eq!(session.events[0].author, "user");
    }
    
    #[tokio::test]
    async fn test_update_session_state() {
        let service = InMemorySessionService::new();
        
        let app_name = "test-app";
        let user_id = "test-user";
        let session_id = "test-session";
        
        // Create a session first
        let mut session = service.create_session(
            app_name,
            user_id,
            None,
            Some(session_id),
        ).await.unwrap();
        
        // Update state
        let mut state = HashMap::new();
        state.insert("test-key".to_string(), serde_json::json!("test-value"));
        
        service.update_session_state(&mut session, state).await.unwrap();
        
        // Check state was updated
        assert!(session.state.contains_key("test-key"));
        assert_eq!(session.state["test-key"], serde_json::json!("test-value"));
    }
    
    #[tokio::test]
    async fn test_close_session() {
        let service = InMemorySessionService::new();
        
        let app_name = "test-app";
        let user_id = "test-user";
        let session_id = "test-session";
        
        // Create a session first
        let mut session = service.create_session(
            app_name,
            user_id,
            None,
            Some(session_id),
        ).await.unwrap();
        
        // Close the session
        service.close_session(&mut session).await.unwrap();
        
        // Check session was closed
        assert_eq!(session.status, SessionStatus::Closed);
    }
    
    #[tokio::test]
    async fn test_list_sessions() {
        let service = InMemorySessionService::new();
        
        let app_name = "test-app";
        let user_id = "test-user";
        
        // Create a few sessions
        service.create_session(
            app_name,
            user_id,
            None,
            Some("session-1"),
        ).await.unwrap();
        
        service.create_session(
            app_name,
            user_id,
            None,
            Some("session-2"),
        ).await.unwrap();
        
        // List sessions
        let response = service.list_sessions(
            app_name,
            user_id,
        ).await.unwrap();
        
        // Check sessions were listed
        assert_eq!(response.sessions.len(), 2);
    }
    
    #[tokio::test]
    async fn test_delete_session() {
        let service = InMemorySessionService::new();
        
        let app_name = "test-app";
        let user_id = "test-user";
        let session_id = "test-session";
        
        // Create a session first
        service.create_session(
            app_name,
            user_id,
            None,
            Some(session_id),
        ).await.unwrap();
        
        // Delete the session
        service.delete_session(
            app_name,
            user_id,
            session_id,
        ).await.unwrap();
        
        // Check session was deleted
        let session = service.get_session(
            app_name,
            user_id,
            session_id,
            None,
        ).await.unwrap();
        
        assert!(session.is_none());
    }
}