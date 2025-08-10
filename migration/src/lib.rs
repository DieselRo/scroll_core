// migration/src/lib.rs
#![warn(unused_imports)]

pub use sea_orm_migration::prelude::*;

mod m20250420_000001_create_scroll_session_table;
mod m20250420_000002_create_scroll_event_table;
mod m20250422_001344_add_session_state_column;
mod m20250501_000001_create_adk_tables;
mod m20250510_000001_create_invocation_ledger;
mod m20250810_000002_create_context_ledger;
mod m20250810_000003_create_trigger_loom_ledger;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250420_000001_create_scroll_session_table::Migration),
            Box::new(m20250420_000002_create_scroll_event_table::Migration),
            Box::new(m20250422_001344_add_session_state_column::Migration),
            Box::new(m20250501_000001_create_adk_tables::Migration),
            Box::new(m20250510_000001_create_invocation_ledger::Migration),
            Box::new(m20250810_000002_create_context_ledger::Migration),
            Box::new(m20250810_000003_create_trigger_loom_ledger::Migration),
        ]
    }
}
