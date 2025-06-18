// src/models/scroll_event.rs
// ===================================================
// SeaORM entity definition for scroll_events table.
// ===================================================

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "scroll_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,

    pub session_id: String,
    pub author: String,
    pub timestamp: i64,

    /// Stored as serialized JSON
    pub content_json: String,
    pub actions_json: String,

    pub partial: bool,
    pub turn_complete: bool,
    pub interrupted: bool,
    pub branch: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm( belongs_to = "super::scroll_session::Entity", from = "Column::SessionId", to = "super::scroll_session::Column::Id" )]
    Session,
}

impl Related<super::scroll_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
