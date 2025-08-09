use chrono::Utc;
use scroll_core::invocation::ledger_service::{start, LedgerEvent};
use scroll_core::sessions::database;
use scroll_core::invocation::ledger::Entity as LedgerEntity;
use sea_orm::{EntityTrait, PaginatorTrait};
use migration::MigratorTrait;

fn mk_event() -> LedgerEvent {
    use scroll_core::core::cost_manager::{CostDecision, InvocationCost};
    use scroll_core::invocation::types::{Invocation, InvocationMode, InvocationTier};
    LedgerEvent {
        invocation: Invocation {
            id: uuid::Uuid::new_v4(),
            phrase: "test".into(),
            invoker: "test".into(),
            invoked: "unit".into(),
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
    let (handle, svc) = start(8, 5);

    // DB not initialized yet: send 10 events, only 5 should be staged
    for _ in 0..10 {
        let _ = handle.try_log(mk_event());
    }

    // Now init DB and run migrations
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let _ = database::init_sqlite_connection("sqlite::memory:").await;
        let _ = migration::Migrator::up(database::get_db_connection(), None).await;
    });

    // Give the worker a moment to flush
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Drop handle to close channel then shutdown
    drop(handle);
    rt.block_on(async move {
        svc.shutdown(std::time::Duration::from_millis(200)).await;
    });

    // Count rows
    let count_rt = tokio::runtime::Runtime::new().unwrap();
    let wrote = count_rt.block_on(async {
        LedgerEntity::find()
            .count(database::get_db_connection())
            .await
            .unwrap()
    });
    // Expect 5 flushed (staging_capacity)
    assert_eq!(wrote, 5);
}

#[test]
fn backpressure_on_small_channel() {
    let (handle, svc) = start(2, 0); // channel=2, no staging
    let mut ok = 0usize;
    let mut full = 0usize;
    for _ in 0..50 {
        match handle.try_log(mk_event()) {
            Ok(()) => ok += 1,
            Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => full += 1,
            Err(_) => {}
        }
    }
    assert!(ok > 0, "some sends should succeed");
    assert!(full > 0, "backpressure should drop when full");

    // init DB so the worker can drain what made it into the channel
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let _ = database::init_sqlite_connection("sqlite::memory:").await;
        let _ = migration::Migrator::up(database::get_db_connection(), None).await;
    });
    std::thread::sleep(std::time::Duration::from_millis(100));
    drop(handle);
    rt.block_on(async move {
        svc.shutdown(std::time::Duration::from_millis(200)).await;
    });
    let count_rt = tokio::runtime::Runtime::new().unwrap();
    let wrote = count_rt.block_on(async {
        LedgerEntity::find()
            .count(database::get_db_connection())
            .await
            .unwrap()
    });
    assert_eq!(wrote as usize, ok, "DB should contain exactly the accepted sends");
}

#[test]
fn happy_path_after_db_ready() {
    // init DB first
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let _ = database::init_sqlite_connection("sqlite::memory:").await;
        let _ = migration::Migrator::up(database::get_db_connection(), None).await;
    });

    let (handle, svc) = start(32, 16);
    for _ in 0..20 {
        let _ = handle.try_log(mk_event());
    }
    std::thread::sleep(std::time::Duration::from_millis(100));
    drop(handle);
    rt.block_on(async move {
        svc.shutdown(std::time::Duration::from_millis(200)).await;
    });
    let count_rt = tokio::runtime::Runtime::new().unwrap();
    let wrote = count_rt.block_on(async {
        LedgerEntity::find()
            .count(database::get_db_connection())
            .await
            .unwrap()
    });
    assert_eq!(wrote, 20);
}

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
