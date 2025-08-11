use assert_cmd::Command;
use scroll_core::entities::open_threads as ot;
use sea_orm::EntityTrait;
use serde_json;
use std::fs;
use tempfile::tempdir;

fn write_scroll(dir: &std::path::Path, name: &str, title: &str) {
    let path = dir.join(name);
    let body = format!(
        "---\n\
title: \"{}\"\n\
scroll_type: Echo\n\
emotion_signature:\n\
  tone: neutral\n\
  emphasis: 0.0\n\
  resonance: balanced\n\
  intensity: 0.0\n\
tags:\n\
  - test\n\
---\n\
\n\
content\n",
        title
    );
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
}

#[test]
fn doc_scan_contradictions_reports_duplicates_and_threads() {
    let dir = tempdir().unwrap();
    let archive = dir.path().join("scrolls");
    fs::create_dir_all(&archive).unwrap();
    write_scroll(&archive, "a.md", "Same Title");
    write_scroll(&archive, "b.md", "Same Title");
    write_scroll(&archive.join("x"), "dup.md", "Unique A");
    write_scroll(&archive.join("y"), "dup.md", "Unique B");

    let mut cmd = Command::cargo_bin("scroll_core").unwrap();
    cmd.current_dir(dir.path())
        .env(
            "SCROLL_CORE_ARCHIVE_DIR",
            archive.to_string_lossy().to_string(),
        )
        .args(["doc", "--action", "scan-contradictions"])
        .assert()
        .success();

    let report_md = dir
        .path()
        .join("docs")
        .join("reference")
        .join("doc-contradictions.md");
    let report_json = dir
        .path()
        .join("docs")
        .join("reference")
        .join("doc-contradictions.json");
    let md = fs::read_to_string(report_md).unwrap();
    assert!(md.contains("duplicate_title"));
    assert!(md.contains("path_collision"));
    let json = fs::read_to_string(report_json).unwrap();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(v
        .as_array()
        .unwrap()
        .iter()
        .any(|f| f["category"] == "duplicate_title"));
    assert!(v
        .as_array()
        .unwrap()
        .iter()
        .any(|f| f["category"] == "path_collision"));

    let db_path = dir.path().join("test.db");
    let db_url = format!("sqlite://{}", db_path.to_string_lossy());
    fs::File::create(&db_path).unwrap();
    let mut cmd = Command::cargo_bin("scroll_core").unwrap();
    cmd.current_dir(dir.path())
        .env(
            "SCROLL_CORE_ARCHIVE_DIR",
            archive.to_string_lossy().to_string(),
        )
        .env("DATABASE_URL", &db_url)
        .args(["doc", "--action", "scan-contradictions", "--fix"])
        .assert()
        .success();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let conn = sea_orm::Database::connect(&db_url).await.unwrap();
        let threads = ot::Entity::find().all(&conn).await.unwrap();
        assert_eq!(threads.len(), 2);
        for t in threads {
            let key = t.dedupe_key.unwrap();
            assert!(key.starts_with("CONTRADUPE|"));
        }
    });
}
