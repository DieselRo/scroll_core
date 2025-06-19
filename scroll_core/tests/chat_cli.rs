use assert_cmd::Command;
use predicates::str::contains;
use sqlx::SqlitePool;

#[tokio::test]
async fn chat_cli_records() {
    let _ = std::fs::remove_file("scroll_core.db");
    let mut cmd = Command::cargo_bin("scroll_core").unwrap();
    cmd.env("SCROLL_CORE_USE_MOCK", "1")
        .args(["chat", "mythscribe", "--stream=false"])
        .write_stdin("ping\nexit\n")
        .assert()
        .success()
        .stdout(contains("pong"));

    let pool = SqlitePool::connect_lazy("sqlite://scroll_core.db").unwrap();
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM scroll_events")
        .fetch_one(&pool)
        .await
        .unwrap();
    let count = row.0;
    assert!(count >= 2);
}
