// scroll_core/src/sessions/database_session_service.rs
// ======================================================
// SeaORM-backed implementation of SessionService trait
// ======================================================

use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait, ColumnTrait, QueryFilter};
use crate::sessions::session::ScrollSession;
use crate::sessions::state::State;
use crate::events::scroll_event::ScrollEvent;
use crate::sessions::session_service::{
    GetSessionConfig,
    ListEventsResponse,
    ListSessionsResponse,
    SessionService,
};
use crate::sessions::error::SessionError;

use crate::models::{scroll_session, scroll_event};
use std::collections::HashMap;
use uuid::Uuid;
use serde_json;

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
    async fn create_session(
        &self,
        app_name: &str,
        user_id: &str,
        state: Option<HashMap<String, String>>,
        session_id: Option<String>,
    ) -> Result<ScrollSession, Box<dyn std::error::Error>> {
        let session = ScrollSession::new(
            session_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            app_name.to_string(),
            user_id.to_string(),
            state.unwrap_or_default(),
        );

        let active_model = scroll_session::ActiveModel {
            id: Set(session.id.clone()),
            app_name: Set(session.app_name.clone()),
            user_id: Set(session.user_id.clone()),
            state_json: Set(serde_json::to_string(&session.state.to_full_map())?),
            last_update_time: Set(session.last_update_time as i64),
            created_at: Set(session.last_update_time as i64),
        };

        active_model.insert(&self.conn).await?;
        Ok(session)
    }

    async fn get_session(
        &self,
        _app_name: &str,
        _user_id: &str,
        id: &str,
        _config: Option<GetSessionConfig>,
    ) -> Result<Option<ScrollSession>, Box<dyn std::error::Error>> {
        let result = scroll_session::Entity::find_by_id(id.to_string())
            .one(&self.conn)
            .await?;

        let result = match result {
            Some(r) => r,
            None => return Ok(None),
        };

        let events = scroll_event::Entity::find()
            .filter(scroll_event::Column::SessionId.eq(id))
            .all(&self.conn)
            .await?;

        Ok(Some(ScrollSession {
            id: result.id,
            app_name: result.app_name,
            user_id: result.user_id,
            state: State::new(),
            events: vec![],
            last_update_time: result.last_update_time as u64,
        }))
    }

    async fn append_event(
        &self,
        _session: &mut ScrollSession,
        _event: ScrollEvent,
    ) -> Result<ScrollEvent, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn list_events(
        &self,
        _app_name: &str,
        _user_id: &str,
        session_id: &str,
    ) -> Result<ListEventsResponse, Box<dyn std::error::Error>> {
        let events = scroll_event::Entity::find()
            .filter(scroll_event::Column::SessionId.eq(session_id))
            .all(&self.conn)
            .await?;

        Ok(ListEventsResponse {
            events: vec![],
            next_page_token: None,
        })
    }

    async fn list_sessions(
        &self,
        _app_name: &str,
        _user_id: &str,
    ) -> Result<ListSessionsResponse, Box<dyn std::error::Error>> {
        let sessions = scroll_session::Entity::find()
            .all(&self.conn)
            .await?;

        Ok(ListSessionsResponse {
            sessions: Vec::new(),
        })
    }

    async fn delete_session(
        &self,
        _app_name: &str,
        _user_id: &str,
        session_id: &str,
        _config: Option<GetSessionConfig>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        scroll_event::Entity::delete_many()
            .filter(scroll_event::Column::SessionId.eq(session_id))
            .exec(&self.conn)
            .await?;

        scroll_session::Entity::delete_by_id(session_id.to_string())
            .exec(&self.conn)
            .await?;

        Ok(())
    }

    async fn close_session(
        &self,
        _session: &mut ScrollSession,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
