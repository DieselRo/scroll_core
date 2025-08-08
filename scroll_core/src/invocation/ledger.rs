//! Records invocation activity to a plain text log for later review.
//! This module is used by the AelrenHerald and InvocationManager to track history.
//! See [Thiren](../AGENTS.md#thiren) for future audit enhancements.
// ===============================
// src/ledger.rs
// ===============================

use crate::core::cost_manager::{CostDecision, InvocationCost};
use crate::invocation::types::Invocation;
use crate::sessions::database::get_db_connection;
use sea_orm::entity::prelude::*;
use sea_orm::Set;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "invocation_ledger")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: uuid::Uuid,
    pub phrase: String,
    pub invoker: String,
    pub invoked: String,
    pub tier: String,
    pub mode: String,
    pub resonance_required: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub cost_system_pressure: f32,
    pub cost_token_pressure: f32,
    pub decision: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn log_invocation_db(
    invocation: &Invocation,
    cost: &InvocationCost,
) -> Result<(), sea_orm::DbErr> {
    let conn = get_db_connection();
    let model = ActiveModel {
        id: Set(invocation.id),
        phrase: Set(invocation.phrase.clone()),
        invoker: Set(invocation.invoker.clone()),
        invoked: Set(invocation.invoked.clone()),
        tier: Set(format!("{:?}", invocation.tier)),
        mode: Set(format!("{:?}", invocation.mode)),
        resonance_required: Set(invocation.resonance_required),
        timestamp: Set(invocation.timestamp),
        cost_system_pressure: Set(cost.cost_profile.system_pressure),
        cost_token_pressure: Set(cost.cost_profile.token_pressure),
        decision: Set(match &cost.decision {
            CostDecision::Allow => "Allow".into(),
            CostDecision::Reject(r) => format!("Reject: {}", r),
            CostDecision::Throttle(x) => format!("Throttle({:.2})", x),
        }),
    };
    let _ = model.insert(conn).await?;
    Ok(())
}
