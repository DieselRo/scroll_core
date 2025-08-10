use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

fn write_scroll(dir: &std::path::Path, name: &str, title: &str) {
    let path = dir.join(name);
    let body = format!(
        "---\\n\n"
        "title: \"{}\"\\n"
        "scroll_type: Echo\\n"
        "emotion_signature:\\n"
        "  tone: neutral\\n"
        "  emphasis: 0.0\\n"
        "  resonance: balanced\\n"
        "  intensity: 0.0\\n"
        "tags:\\n"
        "  - test\\n"
        "---\\n\\n"
        "content\\n",
        title
    );
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
}

#[test]
fn doc_scan_contradictions_reports_duplicates() {
    let dir = tempdir().unwrap();
    let archive = dir.path().join("scrolls");
    fs::create_dir_all(&archive).unwrap();
    write_scroll(&archive, "a.md", "Same Title");
    write_scroll(&archive, "b.md", "Same Title");

    // Run command in temp dir, pointing archive dir via env
    let mut cmd = Command::cargo_bin("scroll_core").unwrap();
    cmd.current_dir(dir.path())
        .env("SCROLL_CORE_ARCHIVE_DIR", archive.to_string_lossy().to_string())
        .args(["doc", "--action", "scan-contradictions"])
        .assert()
        .success();

    let report = dir
        .path()
        .join("docs")
        .join("reference")
        .join("doc-contradictions.md");
    let s = fs::read_to_string(report).unwrap();
    assert!(s.contains("Duplicate Titles"));
}

