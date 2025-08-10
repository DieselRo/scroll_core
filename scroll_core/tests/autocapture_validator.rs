use assert_cmd::Command;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[cfg_attr(target_os = "windows", ignore)]
#[test]
#[ignore]
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

    // 2) Make the same file valid and validate again → closes the thread for that path
    std::fs::write(&invalid, "---\ntitle: NowGood\nscroll_type: Myth\ntags: [ok]\nemotion_signature: { tone: calm, resonance: soft }\n---\nbody").unwrap();
    let mut cmd2 = Command::cargo_bin("scroll_core").unwrap();
    cmd2.env("DATABASE_URL", &url)
        .args(["ritual", "validate", "--file", invalid.to_str().unwrap()])
        .assert()
        .success();

    // 3) Make it invalid again → fail → should reopen
    std::fs::write(&invalid, "---\ntitle: \nscroll_type: Myth\ntags: [bug]\nemotion_signature: { tone: calm, resonance: soft }\n---\nbody").unwrap();
    let mut cmd3 = Command::cargo_bin("scroll_core").unwrap();
    cmd3.env("DATABASE_URL", &url)
        .args(["ritual", "validate", "--file", invalid.to_str().unwrap()])
        .assert()
        .failure();

    // Verify via CLI to avoid cross-process DB pooling issues on Windows
    let output = Command::cargo_bin("scroll_core")
        .unwrap()
        .env("DATABASE_URL", &url)
        .env("SCROLL_CORE_ARCHIVE_DIR", arch.path())
        .args(["open-threads", "--action", "list", "--limit", "50"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let s = String::from_utf8_lossy(&output);
    assert!(s.contains("Validation failed:") && s.contains("bad.md"));
}
