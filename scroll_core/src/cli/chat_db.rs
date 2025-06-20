use chrono::Utc;
use sqlx::Row;
use sqlx::{sqlite::SqliteRow, SqlitePool};
use uuid::Uuid;

pub struct ChatDb {
    pool: SqlitePool,
}

impl ChatDb {
    pub async fn open(path: &str) -> Result<Self, sqlx::Error> {
        let db_url = if path == ":memory:" {
            "sqlite::memory:".to_string()
        } else {
            format!("sqlite://{}?mode=rwc", path)
        };
        let pool = SqlitePool::connect(&db_url).await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS scroll_sessions (id TEXT PRIMARY KEY, created_at REAL);",
        )
        .execute(&pool)
        .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS scroll_events (id TEXT PRIMARY KEY, session_id TEXT, role TEXT, content TEXT, timestamp REAL);"
        ).execute(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn create_session(&self) -> Result<String, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let ts = Utc::now().timestamp_millis() as f64 / 1000.0;
        sqlx::query("INSERT INTO scroll_sessions (id, created_at) VALUES (?, ?)")
            .bind(&id)
            .bind(ts)
            .execute(&self.pool)
            .await?;
        Ok(id)
    }

    pub async fn log_event(
        &self,
        session_id: &str,
        role: &str,
        content: &str,
    ) -> Result<(), sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let ts = Utc::now().timestamp_millis() as f64 / 1000.0;
        sqlx::query("INSERT INTO scroll_events (id, session_id, role, content, timestamp) VALUES (?, ?, ?, ?, ?)")
            .bind(id)
            .bind(session_id)
            .bind(role)
            .bind(content)
            .bind(ts)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn event_count(&self) -> Result<i64, sqlx::Error> {
        let row: SqliteRow = sqlx::query("SELECT COUNT(*) as cnt FROM scroll_events")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.get::<i64, _>("cnt"))
    }
}
