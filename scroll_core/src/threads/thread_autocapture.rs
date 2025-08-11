use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::open_threads as ot;
use crate::notifications::notify_overdue_thread;
use crate::threads::dedupe_service::DedupeService;
use crate::threads::thread_events_service::ThreadEventsService;
use crate::threads::thread_state_service::ThreadStateService;
use crate::threads::types::{Priority, ThreadEventType, ThreadStatus};

fn validator_key(scroll_path: &str) -> String {
    format!("VALIDATOR|{}", scroll_path)
}

pub struct ThreadAutocapture<'a> {
    conn: &'a DatabaseConnection,
}

impl<'a> ThreadAutocapture<'a> {
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn on_validator_failure(
        &self,
        scroll_path: &str,
        title: &str,
    ) -> Result<String, sea_orm::DbErr> {
        let due = chrono::Utc::now() + chrono::Duration::hours(48);
        let dd = DedupeService::new(self.conn);
        dd.dedupe_or_open(
            scroll_path,
            &validator_key(scroll_path),
            title,
            None,
            Priority::Medium,
            None,
            Some(due),
            Some("VALIDATOR"),
            "validator",
        )
        .await
    }

    pub async fn on_validator_pass(&self, scroll_path: &str) -> Result<(), sea_orm::DbErr> {
        let key = validator_key(scroll_path);
        // find any not-closed threads and close them
        let existing = ot::Entity::find()
            .filter(ot::Column::ScrollPath.eq(scroll_path))
            .filter(ot::Column::DedupeKey.eq(&key))
            .filter(ot::Column::Status.ne(ThreadStatus::Closed.to_string()))
            .all(self.conn)
            .await?;
        let svc = ThreadStateService::new(self.conn);
        for t in existing {
            let _ = svc
                .update_status(
                    &t.id,
                    ThreadStatus::Closed,
                    Some("validator pass"),
                    "validator",
                )
                .await?;
        }
        Ok(())
    }

    pub async fn on_validator_contradiction(
        &self,
        scroll_path: &str,
        code: Option<&str>,
        title: &str,
    ) -> Result<String, sea_orm::DbErr> {
        let due = chrono::Utc::now() + chrono::Duration::hours(48);
        let dedupe = if let Some(c) = code {
            format!("CONTRADICTION|{}|{}", scroll_path, c)
        } else {
            validator_key(scroll_path)
        };
        let dd = DedupeService::new(self.conn);
        dd.dedupe_or_open(
            scroll_path,
            &dedupe,
            title,
            None,
            Priority::Medium,
            None,
            Some(due),
            Some("VALIDATOR"),
            "validator",
        )
        .await
    }

    pub async fn on_doc_contradiction(
        &self,
        scroll_path: &str,
        code: &str,
        title: &str,
    ) -> Result<String, sea_orm::DbErr> {
        let due = chrono::Utc::now() + chrono::Duration::hours(48);
        let dedupe = format!("CONTRADUPE|{}|{}", scroll_path, code);
        let dd = DedupeService::new(self.conn);
        dd.dedupe_or_open(
            scroll_path,
            &dedupe,
            title,
            None,
            Priority::Medium,
            None,
            Some(due),
            Some("VALIDATOR"),
            "validator",
        )
        .await
    }

    /// Emits a nudge system note for blocked or overdue threads (no-op placeholder for future automation)
    pub async fn nudge_blocked_or_overdue(&self) -> Result<usize, sea_orm::DbErr> {
        let now = chrono::Utc::now();
        let mut count = 0usize;
        let events = ThreadEventsService::new(self.conn);
        let threads = ot::Entity::find()
            .filter(ot::Column::Status.is_in(vec![
                ThreadStatus::Blocked.to_string(),
                ThreadStatus::Open.to_string(),
                ThreadStatus::InProgress.to_string(),
            ]))
            .all(self.conn)
            .await?;
        for t in threads {
            let overdue = t.due_at.map(|d| d < now).unwrap_or(false);
            let blocked = t.status == ThreadStatus::Blocked.to_string();
            if blocked || overdue {
                let reason = if blocked {
                    "nudge: blocked"
                } else {
                    "nudge: overdue"
                };
                let _ = events
                    .record_event(
                        &t.id,
                        ThreadEventType::SystemNote,
                        "autocapture",
                        Some(reason),
                    )
                    .await?;
                if overdue {
                    let _ = notify_overdue_thread(&t.id, &t.title, &t.scroll_path, Some(reason));
                }
                count += 1;
            }
        }
        Ok(count)
    }
}
