use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "thread_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String, // UUID
    pub thread_id: String,
    pub event_type: String, // COMMENT | STATUS_CHANGE | ASSIGNMENT | TAG_CHANGE | SYSTEM_NOTE
    pub actor: String,
    pub reason: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

