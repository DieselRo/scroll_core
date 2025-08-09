use std::time::Duration;

use migration::{Migrator, MigratorTrait};
use sea_orm::{EntityTrait, PaginatorTrait};
use scroll_core::core::cost_manager::InvocationCost;
use scroll_core::invocation::ledger::Entity as LedgerEntity;
use scroll_core::invocation::ledger_service::{self, LedgerEvent};
use scroll_core::invocation::types::{Invocation, InvocationMode, InvocationTier};
use scroll_core::sessions::database;
use tokio::runtime::Runtime;
use uuid::Uuid;

fn sample_event() -> LedgerEvent {
    let invocation = Invocation {
        id: Uuid::new_v4(),
        phrase: "test".into(),
        invoker: "tester".into(),
        invoked: "construct".into(),
        tier: InvocationTier::True,
        mode: InvocationMode::Read,
        resonance_required: false,
        timestamp: chrono::Utc::now(),
    };
    let cost = InvocationCost::default();
    LedgerEvent { invocation, cost }
}

#[test]
fn ledger_service_behaviors() {
    let rt = Runtime::new().unwrap();

    // scenario 1: buffer before DB ready
    let (handle, service) = ledger_service::start(8, 5);
    for _ in 0..10 {
        handle.try_log(sample_event()).ok();
    }
    rt.block_on(async {
        let _ = database::init_sqlite_connection("sqlite::memory:").await;
        Migrator::up(database::get_db_connection(), None).await.unwrap();
    });
    drop(handle);
    std::thread::sleep(Duration::from_millis(50));
    let conn = database::get_db_connection();
    let count = rt.block_on(async { LedgerEntity::find().count(conn).await.unwrap() });
    assert_eq!(count, 5);
    service.shutdown(Duration::from_millis(50));

    // scenario 2: backpressure on small channel
    rt.block_on(async { LedgerEntity::delete_many().exec(conn).await.unwrap(); });
    let (handle, service) = ledger_service::start(2, 0);
    let mut accepted = 0;
    for _ in 0..10 {
        if handle.try_log(sample_event()).is_ok() {
            accepted += 1;
        }
    }
    drop(handle);
    std::thread::sleep(Duration::from_millis(50));
    assert_eq!(rt.block_on(async { LedgerEntity::find().count(conn).await.unwrap() }), accepted as u64);
    service.shutdown(Duration::from_millis(50));

    // scenario 3: happy path after DB ready
    rt.block_on(async { LedgerEntity::delete_many().exec(conn).await.unwrap(); });
    let (handle, service) = ledger_service::start(8, 5);
    let mut accepted = 0;
    for _ in 0..20 {
        if handle.try_log(sample_event()).is_ok() {
            accepted += 1;
        }
    }
    drop(handle);
    std::thread::sleep(Duration::from_millis(50));
    assert_eq!(rt.block_on(async { LedgerEntity::find().count(conn).await.unwrap() }), accepted as u64);
    service.shutdown(Duration::from_millis(50));
}
