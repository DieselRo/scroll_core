use std::time::Duration;

use chrono::Utc;
use migration::{Migrator, MigratorTrait};
use sea_orm::{EntityTrait, PaginatorTrait};
use tempfile::NamedTempFile;
use uuid::Uuid;

use scroll_core::core::cost_manager::InvocationCost;
use scroll_core::invocation::ledger::Entity as LedgerEntity;
use scroll_core::invocation::ledger_service::{start, LedgerEvent};
use scroll_core::invocation::types::{Invocation, InvocationMode, InvocationTier};
use scroll_core::sessions::database;

fn make_event(i: i32) -> LedgerEvent {
    LedgerEvent {
        invocation: Invocation {
            id: Uuid::new_v4(),
            phrase: format!("p{}", i),
            invoker: "test".into(),
            invoked: "test".into(),
            tier: InvocationTier::True,
            mode: InvocationMode::Read,
            resonance_required: false,
            timestamp: Utc::now(),
        },
        cost: InvocationCost::default(),
    }
}

#[test]
fn buffers_until_db_ready_then_flushes() {
    let (handle, service) = start(8, 5);
    for i in 0..10 {
        let _ = handle.try_log(make_event(i));
    }

    let tmp = NamedTempFile::new().unwrap();
    let db_url = format!("sqlite://{}?mode=rwc", tmp.path().to_str().unwrap());
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        database::init_sqlite_connection(&db_url).await.unwrap();
        Migrator::up(database::get_db_connection(), None).await.unwrap();
    });

    drop(handle);
    service.shutdown(Duration::from_millis(200));

    let count = rt
        .block_on(async {
            LedgerEntity::find()
                .count(database::get_db_connection())
                .await
                .unwrap()
        });
    assert_eq!(count, 5);
}
