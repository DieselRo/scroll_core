use sqlx::SqlitePool;
use tempfile::tempdir;

#[tokio::test(flavor = "multi_thread")]
async fn chat_db_open_variants() -> Result<(), Box<dyn std::error::Error>> {
    // in-memory
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    sqlx::query("CREATE TABLE t (id INTEGER PRIMARY KEY, val TEXT)")
        .execute(&pool)
        .await?;
    sqlx::query("INSERT INTO t (val) VALUES ('ok')")
        .execute(&pool)
        .await?;
    let row: (String,) = sqlx::query_as("SELECT val FROM t")
        .fetch_one(&pool)
        .await?;
    assert_eq!(row.0, "ok");

    // temp file
    let dir = tempdir()?;
    let path = dir.path().join("db.sqlite");
    let url = format!("sqlite://{}?mode=rwc", path.display());
    let pool_file = SqlitePool::connect(&url).await?;
    sqlx::query("CREATE TABLE t (id INTEGER PRIMARY KEY, val TEXT)")
        .execute(&pool_file)
        .await?;
    sqlx::query("INSERT INTO t (val) VALUES ('ok2')")
        .execute(&pool_file)
        .await?;
    let row2: (String,) = sqlx::query_as("SELECT val FROM t")
        .fetch_one(&pool_file)
        .await?;
    assert_eq!(row2.0, "ok2");
    Ok(())
}
