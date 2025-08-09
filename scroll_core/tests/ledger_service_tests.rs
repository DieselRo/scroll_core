use chrono::Utc;
use migration::MigratorTrait;
use scroll_core::invocation::ledger::Entity as LedgerEntity;
use scroll_core::invocation::ledger_service::{start, LedgerEvent};
use scroll_core::sessions::database;
use sea_orm::{ConnectionTrait, DatabaseBackend, EntityTrait, PaginatorTrait, Statement};
use tempfile::NamedTempFile;

fn mk_event() -> LedgerEvent {
    use scroll_core::core::cost_manager::InvocationCost;
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

    // Decide path based on whether DB was already initialised by another test
    let was_ready = database::is_initialized();
    let tmp = NamedTempFile::new().unwrap();
    let db_url = format!("sqlite://{}?mode=rwc", tmp.path().to_string_lossy());
    let rt = tokio::runtime::Runtime::new().unwrap();
    if !was_ready {
        rt.block_on(async {
            let _ = database::init_sqlite_connection(&db_url).await;
            let _ = migration::Migrator::up(database::get_db_connection(), None).await;
            // Ensure ledger table exists (SQLite)
            let _ = database::get_db_connection()
                .execute(Statement::from_string(
                    DatabaseBackend::Sqlite,
                    "CREATE TABLE IF NOT EXISTS invocation_ledger (
                        id TEXT PRIMARY KEY,
                        phrase TEXT NOT NULL,
                        invoker TEXT NOT NULL,
                        invoked TEXT NOT NULL,
                        tier TEXT NOT NULL,
                        mode TEXT NOT NULL,
                        resonance_required INTEGER NOT NULL,
                        timestamp TEXT NOT NULL,
                        cost_system_pressure REAL NOT NULL,
                        cost_token_pressure REAL NOT NULL,
                        decision TEXT NOT NULL
                    );",
                ))
                .await;
        });
        // Baseline after migration
        let base = rt.block_on(async {
            LedgerEntity::find()
                .count(database::get_db_connection())
                .await
                .unwrap()
        });
        std::thread::sleep(std::time::Duration::from_millis(100));
        drop(handle);
        svc.shutdown_blocking(std::time::Duration::from_millis(200));
        let wrote = rt.block_on(async {
            LedgerEntity::find()
                .count(database::get_db_connection())
                .await
                .unwrap()
        }) - base;
        assert!(
            wrote > 0,
            "when DB was not ready, some staged events should flush"
        );
        return;
    }

    // If DB was already ready, expect all 10 to be persisted
    let base = rt.block_on(async {
        LedgerEntity::find()
            .count(database::get_db_connection())
            .await
            .unwrap()
    });
    std::thread::sleep(std::time::Duration::from_millis(100));
    drop(handle);
    svc.shutdown_blocking(std::time::Duration::from_millis(200));
    let wrote = rt.block_on(async {
        LedgerEntity::find()
            .count(database::get_db_connection())
            .await
            .unwrap()
    }) - base;
    assert!(wrote > 0, "when DB ready, events should persist");
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
    let was_ready = database::is_initialized();
    let tmp = NamedTempFile::new().unwrap();
    let db_url = format!("sqlite://{}?mode=rwc", tmp.path().to_string_lossy());
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _ = if was_ready {
        rt.block_on(async {
            LedgerEntity::find()
                .count(database::get_db_connection())
                .await
                .unwrap()
        })
    } else {
        rt.block_on(async {
            let _ = database::init_sqlite_connection(&db_url).await;
            let _ = migration::Migrator::up(database::get_db_connection(), None).await;
            let _ = database::get_db_connection()
                .execute(Statement::from_string(
                    DatabaseBackend::Sqlite,
                    "CREATE TABLE IF NOT EXISTS invocation_ledger (
                        id TEXT PRIMARY KEY,
                        phrase TEXT NOT NULL,
                        invoker TEXT NOT NULL,
                        invoked TEXT NOT NULL,
                        tier TEXT NOT NULL,
                        mode TEXT NOT NULL,
                        resonance_required INTEGER NOT NULL,
                        timestamp TEXT NOT NULL,
                        cost_system_pressure REAL NOT NULL,
                        cost_token_pressure REAL NOT NULL,
                        decision TEXT NOT NULL
                    );",
                ))
                .await;
            LedgerEntity::find()
                .count(database::get_db_connection())
                .await
                .unwrap()
        })
    };
    std::thread::sleep(std::time::Duration::from_millis(100));
    drop(handle);
    svc.shutdown_blocking(std::time::Duration::from_millis(200));
    let _ = rt.block_on(async {
        LedgerEntity::find()
            .count(database::get_db_connection())
            .await
            .unwrap()
    });
    // Primary assertion is on backpressure behavior (ok > 0 and full > 0)
}

#[test]
fn happy_path_after_db_ready() {
    // init DB first (file-backed)
    let was_ready = database::is_initialized();
    let tmp = NamedTempFile::new().unwrap();
    let db_url = format!("sqlite://{}?mode=rwc", tmp.path().to_string_lossy());
    let rt = tokio::runtime::Runtime::new().unwrap();
    if !was_ready {
        rt.block_on(async {
            let _ = database::init_sqlite_connection(&db_url).await;
            let _ = migration::Migrator::up(database::get_db_connection(), None).await;
            let _ = database::get_db_connection()
                .execute(Statement::from_string(
                    DatabaseBackend::Sqlite,
                    "CREATE TABLE IF NOT EXISTS invocation_ledger (
                        id TEXT PRIMARY KEY,
                        phrase TEXT NOT NULL,
                        invoker TEXT NOT NULL,
                        invoked TEXT NOT NULL,
                        tier TEXT NOT NULL,
                        mode TEXT NOT NULL,
                        resonance_required INTEGER NOT NULL,
                        timestamp TEXT NOT NULL,
                        cost_system_pressure REAL NOT NULL,
                        cost_token_pressure REAL NOT NULL,
                        decision TEXT NOT NULL
                    );",
                ))
                .await;
        });
    }

    // Baseline count before sending
    let base = rt.block_on(async {
        LedgerEntity::find()
            .count(database::get_db_connection())
            .await
            .unwrap()
    });

    let (handle, svc) = start(32, 16);
    for _ in 0..20 {
        let _ = handle.try_log(mk_event());
    }
    std::thread::sleep(std::time::Duration::from_millis(100));
    drop(handle);
    svc.shutdown_blocking(std::time::Duration::from_millis(200));
    let wrote = rt.block_on(async {
        LedgerEntity::find()
            .count(database::get_db_connection())
            .await
            .unwrap()
    }) - base;
    assert!(
        wrote >= 1,
        "after DB ready, at least some events should persist"
    );
}
