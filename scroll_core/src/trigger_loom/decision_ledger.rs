use crate::sessions::database;
use sea_orm::{entity::prelude::*, Set};

pub mod ticks {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "trigger_ticks")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: uuid::Uuid,
        pub tick_no: i64,
        pub started_at: chrono::DateTime<chrono::Utc>,
        pub emotions_json: Option<serde_json::Value>,
        pub budget_in: i32,
        pub budget_out: i32,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod decisions {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "trigger_decisions")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: uuid::Uuid,
        pub tick_id: uuid::Uuid,
        pub construct: String,
        pub decision_kind: String,
        pub est_cost_tokens: i32,
        pub budget_remaining: i32,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

#[allow(clippy::too_many_arguments)]
pub async fn insert_tick(
    tick_no: i64,
    started_at: chrono::DateTime<chrono::Utc>,
    emotions_json: Option<serde_json::Value>,
    budget_in: i32,
) -> Result<uuid::Uuid, sea_orm::DbErr> {
    if !database::is_initialized() {
        return Ok(uuid::Uuid::nil());
    }
    let conn = database::get_db_connection();
    let id = uuid::Uuid::new_v4();
    let model = ticks::ActiveModel {
        id: Set(id),
        tick_no: Set(tick_no),
        started_at: Set(started_at),
        emotions_json: Set(emotions_json),
        budget_in: Set(budget_in),
        budget_out: Set(budget_in),
    };
    let _ = ticks::Entity::insert(model).exec(conn).await?;
    Ok(id)
}

pub async fn update_tick_budget_out(
    tick_id: uuid::Uuid,
    budget_out: i32,
) -> Result<(), sea_orm::DbErr> {
    if !database::is_initialized() || tick_id.is_nil() {
        return Ok(());
    }
    use sea_orm::ActiveValue::Unchanged;
    let conn = database::get_db_connection();
    let model = ticks::ActiveModel {
        id: Unchanged(tick_id),
        budget_out: Set(budget_out),
        ..Default::default()
    };
    let _ = ticks::Entity::update(model).exec(conn).await?;
    Ok(())
}

pub async fn insert_decision(
    tick_id: uuid::Uuid,
    construct: &str,
    decision_kind: &str,
    est_cost_tokens: i32,
    budget_remaining: i32,
) -> Result<(), sea_orm::DbErr> {
    if !database::is_initialized() {
        return Ok(());
    }
    let conn = database::get_db_connection();
    let model = decisions::ActiveModel {
        id: Set(uuid::Uuid::new_v4()),
        tick_id: Set(tick_id),
        construct: Set(construct.to_string()),
        decision_kind: Set(decision_kind.to_string()),
        est_cost_tokens: Set(est_cost_tokens),
        budget_remaining: Set(budget_remaining),
    };
    let _ = decisions::Entity::insert(model).exec(conn).await?;
    Ok(())
}
