use std::time::Duration;

use chrono::Utc;
use migration::{Migrator, MigratorTrait};
use sea_orm::{EntityTrait, PaginatorTrait};
use tempfile::NamedTempFile;
use scroll_core::core::cost_manager::InvocationCost;
use scroll_core::invocation::ledger::Entity as LedgerEntity;
use scroll_core::invocation::ledger_service::{start, LedgerEvent};
use scroll_core::invocation::types::{Invocation, InvocationMode, InvocationTier};
use scroll_core::sessions::database::{get_db_connection, init_sqlite_connection};
use uuid::Uuid;

fn sample_event() -> LedgerEvent {
    LedgerEvent {
        invocation: Invocation {
            id: Uuid::new_v4(),
            phrase: "p".into(),
            invoker: "i".into(),
            invoked: "j".into(),
            tier: InvocationTier::Calling,
            mode: InvocationMode::Read,
            resonance_required: false,
            timestamp: Utc::now(),
        },
        cost: InvocationCost::default(),
    }
}

#[test]
fn ledger_service_behaviour() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let tmp = NamedTempFile::new().unwrap();
    let db_url = format!("sqlite://{}", tmp.path().to_string_lossy());

    // start service before DB ready, send events
    let (handle, service) = start(8, 5);
    for _ in 0..10 {
        let _ = handle.try_log(sample_event());
    }
    rt.block_on(async {
        init_sqlite_connection(&db_url).await.unwrap();
        Migrator::up(get_db_connection(), None).await.unwrap();
    });
    drop(handle);
    service.shutdown(Duration::from_secs(1));
    rt.block_on(async {
        let count = LedgerEntity::find()
            .count(get_db_connection())
            .await
            .unwrap();
        assert_eq!(count, 5);
        let _ = LedgerEntity::delete_many().exec(get_db_connection()).await.unwrap();
    });

    // backpressure with small channel
    let (handle, service) = start(2, 0);
    let mut accepted = 0;
    for _ in 0..100 {
        if handle.try_log(sample_event()).is_ok() {
            accepted += 1;
        }
    }
    drop(handle);
    service.shutdown(Duration::from_secs(1));
    rt.block_on(async {
        let count = LedgerEntity::find()
            .count(get_db_connection())
            .await
            .unwrap();
        assert_eq!(count, accepted);
        assert!(accepted < 100);
        let _ = LedgerEntity::delete_many().exec(get_db_connection()).await.unwrap();
    });

    // happy path with DB ready
    let (handle, service) = start(64, 10);
    for _ in 0..20 {
        let _ = handle.try_log(sample_event());
    }
    drop(handle);
    service.shutdown(Duration::from_secs(1));
    rt.block_on(async {
        let count = LedgerEntity::find()
            .count(get_db_connection())
            .await
            .unwrap();
        assert_eq!(count, 20);
    });
}
