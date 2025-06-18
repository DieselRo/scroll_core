// scroll_core/src/sessions/database_session_service.rs
// ======================================================
// SeaORM-backed implementation of SessionService trait
// ======================================================

use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait, ColumnTrait, QueryFilter};
use crate::sessions::session_service::SessionService;
use crate::sessions::session::{ScrollSession, State};
use crate::sessions::event::ScrollEvent;
use crate::sessions::response::{ListEventsResponse, ListSessionsResponse};
use crate::sessions::error::SessionError;

use entity::scroll_session;
use entity::scroll_event;

pub struct DatabaseSessionService {
    pub conn: DatabaseConnection,
}

impl DatabaseSessionService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl SessionService for DatabaseSessionService {
    async fn create_session(&self, session: ScrollSession) -> Result<(), SessionError> {
        let active_model = scroll_session::ActiveModel {
            id: Set(session.id.clone()),
            app_name: Set(session.app_name.clone()),
            user_id: Set(session.user_id.clone()),
            state: Set(session.state.to_string()),
            last_update_time: Set(session.last_update_time as i64),
            ..Default::default()
        };

        active_model.insert(&self.conn).await?;
        Ok(())
    }

    async fn get_session(&self, id: &str) -> Result<ScrollSession, SessionError> {
        let result = scroll_session::Entity::find_by_id(id.to_string())
            .one(&self.conn)
            .await?
            .ok_or(SessionError::NotFound)?;

        let events = scroll_event::Entity::find()
            .filter(scroll_event::Column::SessionId.eq(id))
            .all(&self.conn)
            .await?;

        Ok(ScrollSession {
            id: result.id,
            app_name: result.app_name,
            user_id: result.user_id,
            state: result.state.parse().unwrap_or(State::Idle),
            events: events.into_iter().map(|e| e.into()).collect(),
            last_update_time: result.last_update_time as u64,
        })
    }

    async fn append_event(&self, session_id: &str, event: ScrollEvent) -> Result<(), SessionError> {
        let active_event = scroll_event::ActiveModel {
            session_id: Set(session_id.to_string()),
            timestamp: Set(event.timestamp as i64),
            source: Set(event.source.clone()),
            role: Set(event.role.clone()),
            content: Set(event.content.clone()),
            ..Default::default()
        };

        active_event.insert(&self.conn).await?;
        Ok(())
    }

    async fn list_events(&self, session_id: &str) -> Result<ListEventsResponse, SessionError> {
        let events = scroll_event::Entity::find()
            .filter(scroll_event::Column::SessionId.eq(session_id))
            .all(&self.conn)
            .await?;

        Ok(ListEventsResponse {
            session_id: session_id.to_string(),
            events: events.into_iter().map(|e| e.into()).collect(),
        })
    }

    async fn list_sessions(&self) -> Result<ListSessionsResponse, SessionError> {
        let sessions = scroll_session::Entity::find()
            .all(&self.conn)
            .await?;

        Ok(ListSessionsResponse {
            sessions: sessions
                .into_iter()
                .map(|s| ScrollSession {
                    id: s.id,
                    app_name: s.app_name,
                    user_id: s.user_id,
                    state: s.state.parse().unwrap_or(State::Idle),
                    events: vec![],
                    last_update_time: s.last_update_time as u64,
                })
                .collect(),
        })
    }

    async fn delete_session(&self, session_id: &str) -> Result<(), SessionError> {
        scroll_event::Entity::delete_many()
            .filter(scroll_event::Column::SessionId.eq(session_id))
            .exec(&self.conn)
            .await?;

        scroll_session::Entity::delete_by_id(session_id.to_string())
            .exec(&self.conn)
            .await?;

        Ok(())
    }

    async fn close_session(&self, session_id: &str) -> Result<(), SessionError> {
        let mut session = scroll_session::Entity::find_by_id(session_id.to_string())
            .one(&self.conn)
            .await?
            .ok_or(SessionError::NotFound)?
            .into_active_model();

        session.state = Set(State::Closed.to_string());
        session.update(&self.conn).await?;
        Ok(())
    }
}
