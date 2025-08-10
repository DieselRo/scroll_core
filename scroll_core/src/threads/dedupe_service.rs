use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::open_threads as ot;
use crate::threads::thread_state_service::ThreadStateService;
use crate::threads::types::{Priority, ThreadStatus};

pub struct DedupeService<'a> {
    conn: &'a DatabaseConnection,
}

impl<'a> DedupeService<'a> {
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn dedupe_or_open(
        &self,
        scroll_path: &str,
        dedupe_key: &str,
        title: &str,
        assignee: Option<&str>,
        priority: Priority,
        tags: Option<Vec<String>>,
        due_at: Option<chrono::DateTime<chrono::Utc>>,
        source: Option<&str>,
        actor: &str,
    ) -> Result<String, sea_orm::DbErr> {
        // look for existing OPEN/IN_PROGRESS/BLOCKED with same scroll_path + dedupe_key
        let existing = ot::Entity::find()
            .filter(ot::Column::ScrollPath.eq(scroll_path))
            .filter(ot::Column::DedupeKey.eq(dedupe_key))
            .filter(ot::Column::Status.is_in(vec![
                ThreadStatus::Open.to_string(),
                ThreadStatus::InProgress.to_string(),
                ThreadStatus::Blocked.to_string(),
            ]))
            .one(self.conn)
            .await?;
        if let Some(model) = existing {
            return Ok(model.id);
        }
        // If closed exists, reopen it
        let closed = ot::Entity::find()
            .filter(ot::Column::ScrollPath.eq(scroll_path))
            .filter(ot::Column::DedupeKey.eq(dedupe_key))
            .filter(ot::Column::Status.eq(ThreadStatus::Closed.to_string()))
            .one(self.conn)
            .await?;
        if let Some(model) = closed {
            let svc = ThreadStateService::new(self.conn);
            let _ = svc
                .update_status(
                    &model.id,
                    ThreadStatus::Open,
                    Some("reopen after failure"),
                    actor,
                )
                .await?;
            return Ok(model.id);
        }
        let svc = ThreadStateService::new(self.conn);
        let mut model = svc
            .create(
                scroll_path,
                title,
                assignee,
                priority,
                tags,
                due_at,
                source,
                actor,
            )
            .await?;
        // Update dedupe key
        use sea_orm::{ActiveModelTrait, Set};
        let mut active: ot::ActiveModel = model.clone().into();
        active.dedupe_key = Set(Some(dedupe_key.to_string()));
        model = active.update(self.conn).await?;
        Ok(model.id)
    }
}
