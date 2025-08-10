use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "open_threads")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String, // store UUID as hyphenated string
    pub scroll_path: String,
    pub title: String,
    pub status: String, // OPEN | IN_PROGRESS | BLOCKED | CLOSED
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub last_event_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

