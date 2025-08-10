use assert_cmd::Command;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[test]
fn validator_autocapture_flow() {
    // Use a temp DB file to persist across CLI calls
    let tmp = tempfile::tempdir().unwrap();
    let db_path = tmp.path().join("auto.sqlite");
    let url = format!("sqlite://{}?mode=rwc", db_path.display());
    // Run migrations once
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let _ = scroll_core::sessions::database::init_sqlite_connection(&url).await;
        let _ = Migrator::up(scroll_core::sessions::database::get_db_connection(), None).await;
    });

    // Place a minimal valid/invalid scroll in a temp archive
    let arch = tempfile::tempdir().unwrap();
    std::env::set_var("SCROLL_CORE_ARCHIVE_DIR", arch.path());
    // Invalid scroll (missing title)
    let invalid = arch.path().join("bad.md");
    std::fs::write(&invalid, "---\ntitle: \nscroll_type: Myth\ntags: [bug]\nemotion_signature: { tone: calm, resonance: soft }\n---\nbody").unwrap();
    // Valid scroll
    let valid = arch.path().join("good.md");
    std::fs::write(&valid, "---\ntitle: Good\nscroll_type: Myth\ntags: [ok]\nemotion_signature: { tone: calm, resonance: soft }\n---\nbody").unwrap();

    // 1) Validate invalid → opens a thread
    let mut cmd = Command::cargo_bin("scroll_core").unwrap();
    cmd.env("DATABASE_URL", &url)
        .args(["ritual", "validate", "--file", invalid.to_str().unwrap()])
        .assert()
        .failure();

    // 2) Validate valid → closes any open thread for the path
    let mut cmd2 = Command::cargo_bin("scroll_core").unwrap();
    cmd2.env("DATABASE_URL", &url)
        .args(["ritual", "validate", "--file", valid.to_str().unwrap()])
        .assert()
        .success();

    // 3) Fail again on the same path → should reopen (increment reopened_count)
    let mut cmd3 = Command::cargo_bin("scroll_core").unwrap();
    cmd3.env("DATABASE_URL", &url)
        .args(["ritual", "validate", "--file", invalid.to_str().unwrap()])
        .assert()
        .failure();

    // Verify there is a thread marked OPEN for invalid path and reopened_count >= 1
    let conn = scroll_core::sessions::database::get_db_connection().clone();
    let rows = rt.block_on(async move {
        use scroll_core::entities::open_threads as ot;
        ot::Entity::find()
            .filter(ot::Column::ScrollPath.eq(invalid.to_string_lossy().to_string()))
            .all(&conn)
            .await
            .unwrap()
    });
    assert!(!rows.is_empty());
    let open = rows.iter().find(|r| r.status == "OPEN").unwrap();
    assert!(open.reopened_count >= 1);
}

