use sea_orm::{DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use crate::entities::thread_events as te;
use crate::threads::types::ThreadEventType;

pub struct ThreadEventsService<'a> {
    conn: &'a DatabaseConnection,
}

impl<'a> ThreadEventsService<'a> {
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn record_event(
        &self,
        thread_id: &str,
        event_type: ThreadEventType,
        actor: &str,
        reason: Option<&str>,
    ) -> Result<te::Model, sea_orm::DbErr> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4().to_string();
        let rec = te::ActiveModel {
            id: Set(id.clone()),
            thread_id: Set(thread_id.to_string()),
            event_type: Set(event_type.to_string()),
            actor: Set(actor.to_string()),
            reason: Set(reason.map(|s| s.to_string())),
            created_at: Set(now),
        };
        te::Entity::insert(rec).exec(self.conn).await?;
        Ok(te::Model {
            id,
            thread_id: thread_id.to_string(),
            event_type: event_type.to_string(),
            actor: actor.to_string(),
            reason: reason.map(|s| s.to_string()),
            created_at: now,
        })
    }
}

