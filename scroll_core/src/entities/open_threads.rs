use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "open_threads")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String, // store UUID as hyphenated string
    pub scroll_path: String,
    pub title: String,
    pub status: String, // OPEN | IN_PROGRESS | BLOCKED | CLOSED
    pub assignee: Option<String>,
    pub priority: String, // LOW | MEDIUM | HIGH
    pub tags: Option<String>, // comma-separated normalized
    pub due_at: Option<chrono::DateTime<chrono::Utc>>,
    pub source: Option<String>,
    pub reopened_count: i32,
    pub dedupe_key: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub last_event_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
