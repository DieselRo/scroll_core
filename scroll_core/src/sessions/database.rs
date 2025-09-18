// src/database.rs
// ===================================================
// Initializes SeaORM connection to SQLite database.
// ===================================================

use once_cell::sync::OnceCell;
use sea_orm::{Database, DbConn, DbErr};
use std::sync::atomic::{AtomicBool, Ordering};

use migration::MigratorTrait;

static DB_CONNECTION: OnceCell<DbConn> = OnceCell::new();
static DB_READY: AtomicBool = AtomicBool::new(false);

/// Initializes and stores a global database connection pool.
///
/// Call this once on application startup. Subsequent calls will fail
/// if the connection has already been initialized.
///
/// # Arguments
/// * `db_url` - The database URL (e.g., "sqlite://scroll_core.db")
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(DbErr)` if connection fails or already initialized
pub async fn init_sqlite_connection(db_url: &str) -> Result<(), DbErr> {
    let conn = Database::connect(db_url).await?;
    DB_CONNECTION.set(conn).map_err(|_| {
        DbErr::Conn(sea_orm::RuntimeErr::Internal(
            "Database already initialized".to_owned(),
        ))
    })
}

/// Gets a reference to the global DB connection.
///
/// # Panics
/// Panics if the database connection has not been initialized.
/// Make sure `init_sqlite_connection()` has been called before using this.
pub fn get_db_connection() -> &'static DbConn {
    DB_CONNECTION
        .get()
        .expect("Database not initialized. Call init_sqlite_connection() first.")
}

/// Returns true if the global database connection has been initialized.
pub fn is_initialized() -> bool {
    DB_CONNECTION.get().is_some()
}

/// Runs migrations and marks the database as ready for writers.
/// Safe to call multiple times; subsequent calls are no-ops.
pub async fn ensure_ready_with_url(db_url: &str) -> Result<&'static DbConn, DbErr> {
    if !is_initialized() {
        init_sqlite_connection(db_url).await?;
    }
    let conn = get_db_connection();
    // Best-effort migrations; if another caller races, Migrator::up is idempotent
    let _ = migration::Migrator::up(conn, None).await;
    DB_READY.store(true, Ordering::SeqCst);
    Ok(conn)
}

/// Reads DATABASE_URL from env, falls back to sqlite file in CWD.
pub async fn ensure_ready_from_env() -> Result<&'static DbConn, DbErr> {
    let raw = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://scroll_core.db".into());
    // Normalize: strip query
    let url = match raw.find('?') {
        Some(i) => &raw[..i],
        None => &raw,
    };
    ensure_ready_with_url(url).await
}

/// Returns true once migrations have been applied.
pub fn is_ready() -> bool {
    DB_READY.load(Ordering::SeqCst)
}
