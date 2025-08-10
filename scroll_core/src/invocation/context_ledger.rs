use sea_orm::entity::prelude::*;
use sea_orm::Set;
use uuid::Uuid;

use crate::core::context_decisions::ContextBuildReport;
use crate::sessions::database::get_db_connection;

pub mod frame {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "context_frame_ledger")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub frame_id: uuid::Uuid,
        pub construct: String,
        pub max_tokens: i32,
        pub max_items: i32,
        pub min_relevance: f32,
        pub half_life_hours: f32,
        pub total_candidates: i32,
        pub included_count: i32,
        pub excluded_count: i32,
        pub build_ms: i64,
        pub timestamp: chrono::DateTime<chrono::Utc>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod candidate {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "context_candidate_ledger")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: uuid::Uuid,
        pub frame_id: uuid::Uuid,
        pub construct: String,
        pub candidate_path: Option<String>,
        pub included: bool,
        pub reason: String,
        pub score: f32,
        pub recency_hours: f32,
        pub running_tokens: i32,
        pub max_tokens: i32,
        pub timestamp: chrono::DateTime<chrono::Utc>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub async fn log_context_report(report: &ContextBuildReport) -> Result<(), sea_orm::DbErr> {
    // Do nothing if DB is not initialized
    if !crate::sessions::database::is_initialized() {
        return Ok(());
    }
    let conn = get_db_connection();
    let summary = &report.summary;
    let frame = frame::ActiveModel {
        frame_id: Set(summary.frame_id),
        construct: Set(summary.construct.clone()),
        max_tokens: Set(summary.max_tokens as i32),
        max_items: Set(summary.max_items as i32),
        min_relevance: Set(summary.min_relevance),
        half_life_hours: Set(summary.half_life_hours),
        total_candidates: Set(summary.total_candidates as i32),
        included_count: Set(summary.included_count as i32),
        excluded_count: Set(summary.excluded_count as i32),
        build_ms: Set(summary.build_ms as i64),
        timestamp: Set(chrono::Utc::now()),
    };
    let _ = frame.insert(conn).await?;

    // Optional detail rows
    let verbose = std::env::var("SC_CONTEXT_DECISIONS_VERBOSE")
        .ok()
        .as_deref()
        == Some("1");
    if verbose {
        for d in &report.decisions {
            let rec = candidate::ActiveModel {
                id: Set(Uuid::new_v4()),
                frame_id: Set(d.frame_id),
                construct: Set(d.construct.clone()),
                candidate_path: Set(d.candidate_path.clone()),
                included: Set(d.included),
                reason: Set(d.reason.clone()),
                score: Set(d.score),
                recency_hours: Set(d.recency_hours),
                running_tokens: Set(d.running_tokens as i32),
                max_tokens: Set(d.max_tokens as i32),
                timestamp: Set(chrono::Utc::now()),
            };
            let _ = rec.insert(conn).await?;
        }
    }
    Ok(())
}
