// src/database.rs
// ===================================================
// Initializes SeaORM connection to SQLite database.
// ===================================================

use sea_orm::{Database, DbConn, DbErr};
use once_cell::sync::OnceCell;

static DB_CONNECTION: OnceCell<DbConn> = OnceCell::new();

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
    DB_CONNECTION
        .set(conn)
        .map_err(|_| DbErr::Conn("Database already initialized".into()))
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
