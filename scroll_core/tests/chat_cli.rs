use assert_cmd::Command;
use predicates::str::contains;
use scroll_core::invocation::ledger::Entity as LedgerEntity;
use sea_orm::EntityTrait;
use sea_orm::PaginatorTrait;
use std::fs;
use tempfile::tempdir;

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
        .env("SCROLL_CI", "1")
        .env("SCROLL_CORE_ARCHIVE_DIR", archive)
        .env(
            "DATABASE_URL",
            format!("sqlite://{}?mode=rwc", db_path.to_str().unwrap()),
        )
        .current_dir(archive)
        .args(["chat", "mythscribe", "--no-stream"])
        .write_stdin("ping\nexit\n")
        .assert()
        .success()
        .stdout(contains("pong"));

    // Connect via SeaORM session helper and count rows
    scroll_core::sessions::database::init_sqlite_connection(&format!(
        "sqlite://{}?mode=rwc",
        db_path.to_str().unwrap()
    ))
    .await
    .unwrap();
    let count = LedgerEntity::find()
        .count(scroll_core::sessions::database::get_db_connection())
        .await
        .unwrap();
    assert!(count >= 1);
}
