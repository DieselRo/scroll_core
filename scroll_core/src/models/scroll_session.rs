// src/models/scroll_session.rs
// ===================================================
// SeaORM entity definition for scroll_sessions table.
// ===================================================

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "scroll_sessions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,

    pub app_name: String,
    pub user_id: String,
    pub created_at: i64,
    pub last_update_time: i64,
    pub state_json: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::scroll_event::Entity")]
    ScrollEvent,
}

impl Related<super::scroll_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ScrollEvent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
