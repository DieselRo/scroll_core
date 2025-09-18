use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::str::FromStr;
use uuid::Uuid;

use crate::entities::open_threads as ot;
use crate::notifications::{notify_event, NotificationEvent, NotificationKind};
use crate::threads::thread_events_service::ThreadEventsService;
use crate::threads::types::{normalize_tags, tags_to_db, Priority, ThreadEventType, ThreadStatus};

pub struct ThreadStateService<'a> {
    conn: &'a DatabaseConnection,
}

impl<'a> ThreadStateService<'a> {
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn create(
        &self,
        scroll_path: &str,
        title: &str,
        assignee: Option<&str>,
        priority: Priority,
        tags: Option<Vec<String>>,
        due_at: Option<chrono::DateTime<chrono::Utc>>,
        source: Option<&str>,
        actor: &str,
    ) -> Result<ot::Model, sea_orm::DbErr> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4().to_string();
        let status = ThreadStatus::Open;
        let tags_norm = tags.map(|t| tags_to_db(&normalize_tags(t)));
        let rec = ot::ActiveModel {
            id: Set(id.clone()),
            scroll_path: Set(scroll_path.to_string()),
            title: Set(title.to_string()),
            status: Set(status.to_string()),
            assignee: Set(assignee.map(|s| s.to_string())),
            priority: Set(priority.to_string()),
            tags: Set(tags_norm),
            due_at: Set(due_at),
            source: Set(source.map(|s| s.to_string())),
            reopened_count: Set(0),
            dedupe_key: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
            last_event_id: Set(None),
        };
        ot::Entity::insert(rec).exec(self.conn).await?;
        let events = ThreadEventsService::new(self.conn);
        let ev = events
            .record_event(
                &id,
                ThreadEventType::SystemNote,
                actor,
                Some("thread created"),
            )
            .await?;
        // Update last_event_id
        let mut loaded = ot::Entity::find_by_id(id.clone())
            .one(self.conn)
            .await?
            .expect("inserted thread not found");
        let mut act: ot::ActiveModel = loaded.clone().into();
        let ev_id = ev.id.clone();
        act.last_event_id = Set(Some(ev_id.clone()));
        act.update(self.conn).await?;
        loaded.last_event_id = Some(ev_id);

        // Send notification: ThreadCreated
        let mut note = NotificationEvent::new(
            NotificationKind::ThreadCreated,
            id.clone(),
            title.to_string(),
            scroll_path.to_string(),
        );
        note.assignee = assignee.map(|s| s.to_string());
        note.priority = Some(priority.to_string());
        let _ = notify_event(note);
        Ok(loaded)
    }

    pub async fn update_status(
        &self,
        id: &str,
        new_status: ThreadStatus,
        reason: Option<&str>,
        actor: &str,
    ) -> Result<ot::Model, sea_orm::DbErr> {
        let model = ot::Entity::find_by_id(id.to_string())
            .one(self.conn)
            .await?
            .ok_or_else(|| sea_orm::DbErr::RecordNotFound(format!("thread {} not found", id)))?;
        let old_status = ThreadStatus::from_str(&model.status).unwrap_or(ThreadStatus::Open);
        if !ThreadStatus::allowed_transition(old_status, new_status) {
            return Err(sea_orm::DbErr::Custom(format!(
                "illegal transition: {} -> {}",
                old_status, new_status
            )));
        }
        let mut active: ot::ActiveModel = model.clone().into();
        active.status = Set(new_status.to_string());
        if matches!(
            (old_status, new_status),
            (ThreadStatus::Closed, ThreadStatus::Open)
        ) {
            active.reopened_count = Set(model.reopened_count + 1);
        }
        active.updated_at = Set(chrono::Utc::now());

        let events = ThreadEventsService::new(self.conn);
        let ev = events
            .record_event(
                id,
                ThreadEventType::StatusChange,
                actor,
                reason.or(Some("status change")),
            )
            .await?;
        active.last_event_id = Set(Some(ev.id));
        let updated = active.update(self.conn).await?;

        // Send notification: StatusChanged with flap/rate control in hub
        let mut note = NotificationEvent::new(
            NotificationKind::StatusChanged,
            id.to_string(),
            updated.title.clone(),
            updated.scroll_path.clone(),
        );
        note.status = Some(new_status.to_string());
        note.reason = reason.map(|s| s.to_string());
        let _ = notify_event(note);
        Ok(updated)
    }

    pub async fn assign(
        &self,
        id: &str,
        assignee: Option<&str>,
        actor: &str,
    ) -> Result<ot::Model, sea_orm::DbErr> {
        let model = ot::Entity::find_by_id(id.to_string())
            .one(self.conn)
            .await?
            .ok_or_else(|| sea_orm::DbErr::RecordNotFound(format!("thread {} not found", id)))?;
        let mut active: ot::ActiveModel = model.into();
        active.assignee = Set(assignee.map(|s| s.to_string()));
        active.updated_at = Set(chrono::Utc::now());

        let events = ThreadEventsService::new(self.conn);
        let ev = events
            .record_event(
                id,
                ThreadEventType::Assignment,
                actor,
                Some(assignee.unwrap_or("<unassigned>")),
            )
            .await?;
        active.last_event_id = Set(Some(ev.id));
        let updated = active.update(self.conn).await?;

        // Send notification: AssignmentChanged
        let mut note = NotificationEvent::new(
            NotificationKind::AssignmentChanged,
            id.to_string(),
            updated.title.clone(),
            updated.scroll_path.clone(),
        );
        note.assignee = assignee.map(|s| s.to_string());
        note.reason = Some("assignment change".into());
        let _ = notify_event(note);
        Ok(updated)
    }

    pub async fn set_priority(
        &self,
        id: &str,
        priority: Priority,
        actor: &str,
    ) -> Result<ot::Model, sea_orm::DbErr> {
        let model = ot::Entity::find_by_id(id.to_string())
            .one(self.conn)
            .await?
            .ok_or_else(|| sea_orm::DbErr::RecordNotFound(format!("thread {} not found", id)))?;
        let oldp = model.priority.clone();
        let mut active: ot::ActiveModel = model.into();
        let newp = priority.to_string();
        active.priority = Set(newp.clone());
        active.updated_at = Set(chrono::Utc::now());

        let events = ThreadEventsService::new(self.conn);
        let ev = events
            .record_event(
                id,
                ThreadEventType::SystemNote,
                actor,
                Some(&format!("priority={}", priority)),
            )
            .await?;
        active.last_event_id = Set(Some(ev.id));
        let updated = active.update(self.conn).await?;

        // Only notify when moving to HIGH
        if newp == "HIGH" && oldp != "HIGH" {
            let mut note = NotificationEvent::new(
                NotificationKind::StatusChanged,
                id.to_string(),
                updated.title.clone(),
                updated.scroll_path.clone(),
            );
            note.priority = Some(newp);
            note.reason = Some("priority escalated".into());
            let _ = notify_event(note);
        }
        Ok(updated)
    }

    pub async fn set_tags(
        &self,
        id: &str,
        tags: Vec<String>,
        actor: &str,
    ) -> Result<ot::Model, sea_orm::DbErr> {
        let model = ot::Entity::find_by_id(id.to_string())
            .one(self.conn)
            .await?
            .ok_or_else(|| sea_orm::DbErr::RecordNotFound(format!("thread {} not found", id)))?;
        let mut active: ot::ActiveModel = model.into();
        let normalized = tags_to_db(&normalize_tags(tags));
        active.tags = Set(if normalized.is_empty() {
            None
        } else {
            Some(normalized.clone())
        });
        active.updated_at = Set(chrono::Utc::now());

        let events = ThreadEventsService::new(self.conn);
        let ev = events
            .record_event(id, ThreadEventType::TagChange, actor, Some(&normalized))
            .await?;
        active.last_event_id = Set(Some(ev.id));
        let updated = active.update(self.conn).await?;
        Ok(updated)
    }

    pub async fn list(
        &self,
        status: Option<ThreadStatus>,
        scroll_path: Option<&str>,
        limit: Option<u64>,
        assignee: Option<&str>,
        priority: Option<Priority>,
        tags: Option<Vec<String>>,
        overdue_only: bool,
        sort: Option<&str>,
    ) -> Result<Vec<ot::Model>, sea_orm::DbErr> {
        use sea_orm::{QueryOrder, QuerySelect};
        let mut q = ot::Entity::find();
        if let Some(s) = status {
            q = q.filter(ot::Column::Status.eq(s.to_string()));
        }
        if let Some(sp) = scroll_path {
            q = q.filter(ot::Column::ScrollPath.eq(sp));
        }
        if let Some(a) = assignee {
            q = q.filter(ot::Column::Assignee.eq(a));
        }
        if let Some(p) = priority {
            q = q.filter(ot::Column::Priority.eq(p.to_string()));
        }
        if let Some(ts) = tags {
            for t in ts {
                let patt = format!("%{}%", t.to_ascii_lowercase());
                q = q.filter(ot::Column::Tags.like(patt));
            }
        }
        if overdue_only {
            q = q.filter(ot::Column::DueAt.lte(chrono::Utc::now()));
            q = q.filter(ot::Column::Status.ne(ThreadStatus::Closed.to_string()));
        }
        q = q
            .order_by_asc(ot::Column::CreatedAt)
            .order_by_asc(ot::Column::Id);
        if let Some(key) = sort {
            match key {
                "created" => {
                    q = q
                        .order_by_asc(ot::Column::CreatedAt)
                        .order_by_asc(ot::Column::Id);
                }
                "updated" => {
                    q = q
                        .order_by_desc(ot::Column::UpdatedAt)
                        .order_by_asc(ot::Column::Id);
                }
                "priority" => {
                    // ORDER BY priority custom mapping: HIGH > MEDIUM > LOW
                    // Use simple desc alphabetical since our labels sort H > M > L by default
                    q = q
                        .order_by_desc(ot::Column::Priority)
                        .order_by_asc(ot::Column::CreatedAt);
                }
                _ => {}
            }
        }
        if let Some(l) = limit {
            q = q.limit(l);
        }
        q.all(self.conn).await
    }
}
