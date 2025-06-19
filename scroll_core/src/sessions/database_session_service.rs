// scroll_core/src/sessions/database_session_service.rs
// ======================================================
// SeaORM-backed implementation of SessionService trait
// ======================================================

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use std::collections::HashMap;
use crate::sessions::session_service::{
    GetSessionConfig, ListEventsResponse, ListSessionsResponse, SessionService,
};
use crate::sessions::session::ScrollSession;
use crate::sessions::state::State;
use crate::events::ScrollEvent;

use crate::models::{scroll_event, scroll_session};

pub struct DatabaseSessionService {
    pub conn: DatabaseConnection,
}

impl DatabaseSessionService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

impl SessionService for DatabaseSessionService {
    fn create_session(
        &self,
        _app_name: &str,
        _user_id: &str,
        _state: Option<HashMap<String, String>>,
        _session_id: Option<String>,
    ) -> Result<ScrollSession, Box<dyn std::error::Error>> {
        todo!()
    }

    fn get_session(
        &self,
        _app_name: &str,
        _user_id: &str,
        _id: &str,
        _config: Option<GetSessionConfig>,
    ) -> Result<Option<ScrollSession>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn append_event(
        &self,
        _session: &mut ScrollSession,
        _event: ScrollEvent,
    ) -> Result<ScrollEvent, Box<dyn std::error::Error>> {
        todo!()
    }

    fn list_events(
        &self,
        _app_name: &str,
        _user_id: &str,
        _session_id: &str,
    ) -> Result<ListEventsResponse, Box<dyn std::error::Error>> {
        todo!()
    }

    fn list_sessions(
        &self,
        _app_name: &str,
        _user_id: &str,
    ) -> Result<ListSessionsResponse, Box<dyn std::error::Error>> {
        todo!()
    }

    fn delete_session(
        &self,
        _app_name: &str,
        _user_id: &str,
        _session_id: &str,
        _config: Option<GetSessionConfig>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn close_session(
        &self,
        _session: &mut ScrollSession,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
