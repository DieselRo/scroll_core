use assert_cmd::Command;
use predicates::str::contains;
use sqlx::SqlitePool;
use tempfile::tempdir;
use std::fs;

#[tokio::test]
async fn chat_cli_records() {
    let dir = tempdir().unwrap();
    let archive = dir.path();
    let db_path = archive.join("scroll_core.db");

    fs::write(
        archive.join("rust.md"),
        "---\ntitle: Rust\nscroll_type: Canon\nemotion_signature:\n  tone: calm\n  emphasis: 0.5\n  resonance: gentle\ntags: [rust]\n---\nRust body.\n",
    )
    .unwrap();
    fs::write(
        archive.join("cook.md"),
        "---\ntitle: Cook\nscroll_type: Canon\nemotion_signature:\n  tone: calm\n  emphasis: 0.5\n  resonance: gentle\ntags: [cook]\n---\nCook body.\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("scroll_core").unwrap();
    cmd.env("SCROLL_CORE_USE_MOCK", "1")
        .env("SCROLL_CORE_ARCHIVE_DIR", archive)
        .current_dir(archive)
        .args(["chat", "mythscribe", "--stream=false"])
        .write_stdin("ping\nexit\n")
        .assert()
        .success()
        .stdout(contains("pong"));

    let pool = SqlitePool::connect_lazy(
        &format!("sqlite://{}", db_path.to_str().unwrap()),
    )
    .unwrap();
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM scroll_events")
        .fetch_one(&pool)
        .await
        .unwrap();
    let count = row.0;
    assert!(count >= 2);
}
