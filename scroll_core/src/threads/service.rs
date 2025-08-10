use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::entities::open_threads as ot;

pub struct ThreadsService<'a> {
    conn: &'a DatabaseConnection,
}

impl<'a> ThreadsService<'a> {
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        Self { conn }
    }

    // Convenience helper to open a validation-related thread
    pub async fn open_for_validation(
        &self,
        scroll_path: &str,
        title: &str,
        last_event_id: Option<&str>,
        assignee: Option<&str>,
    ) -> Result<ot::Model, sea_orm::DbErr> {
        let now = chrono::Utc::now();
        let mut title_full = title.to_string();
        if let Some(a) = assignee {
            if !a.is_empty() {
                title_full.push_str(&format!(" [assignee: {}]", a));
            }
        }
        let rec = ot::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            scroll_path: Set(scroll_path.to_string()),
            title: Set(title_full.clone()),
            status: Set("OPEN".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
            last_event_id: Set(last_event_id.map(|s| s.to_string())),
        };
        let id = rec.id.clone().unwrap();
        // Use exec to avoid RETURNING limitations on SQLite
        let _ = ot::Entity::insert(rec).exec(self.conn).await?;
        Ok(ot::Model {
            id,
            scroll_path: scroll_path.to_string(),
            title: title_full,
            status: "OPEN".to_string(),
            created_at: now,
            updated_at: now,
            last_event_id: last_event_id.map(|s| s.to_string()),
        })
    }

    pub async fn close(
        &self,
        id: &str,
        reason: Option<&str>,
        last_event_id: Option<&str>,
    ) -> Result<u64, sea_orm::DbErr> {
        // Load existing
        if let Some(found) = ot::Entity::find_by_id(id.to_string())
            .one(self.conn)
            .await?
        {
            let mut active: ot::ActiveModel = found.into();
            active.status = Set("CLOSED".into());
            if let Some(r) = reason {
                if !r.is_empty() {
                    let new_title = format!("{} (closed: {} )", active.title.clone().unwrap(), r);
                    active.title = Set(new_title);
                }
            }
            active.updated_at = Set(chrono::Utc::now());
            if let Some(ev) = last_event_id {
                active.last_event_id = Set(Some(ev.to_string()));
            }
            let _ = active.update(self.conn).await?;
            Ok(1)
        } else {
            Ok(0)
        }
    }

    pub async fn list(
        &self,
        status: Option<&str>,
        scroll_path: Option<&str>,
        limit: Option<u64>,
    ) -> Result<Vec<ot::Model>, sea_orm::DbErr> {
        use sea_orm::{QueryOrder, QuerySelect};
        let mut q = ot::Entity::find();
        if let Some(s) = status {
            q = q.filter(ot::Column::Status.eq(s));
        }
        if let Some(sp) = scroll_path {
            q = q.filter(ot::Column::ScrollPath.eq(sp));
        }
        // Deterministic ordering
        q = q.order_by_asc(ot::Column::CreatedAt).order_by_asc(ot::Column::Id);
        if let Some(l) = limit {
            q = q.limit(l);
        }
        q.all(self.conn).await
    }
}
