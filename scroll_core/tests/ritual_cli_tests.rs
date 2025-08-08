use assert_cmd::Command;
use std::fs;

#[test]
fn ritual_write_can_update_index() {
    let dir = tempfile::tempdir().unwrap();
    let archive = dir.path();
    fs::create_dir_all(archive).unwrap();
    let file = "alpha.md";
    fs::write(
        archive.join(file),
        "---\ntitle: Alpha\nscroll_type: Canon\nemotion_signature:\n  tone: calm\n  emphasis: 0.1\n  resonance: gentle\ntags: [a]\n---\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("scroll_core").unwrap();
    cmd.env("SCROLL_CI", "1")
        .env("SCROLL_CORE_ARCHIVE_DIR", archive)
        .args([
            "ritual",
            "--action",
            "write",
            "--file",
            file,
            "--update-index",
        ])
        .assert()
        .success();

    let idx = archive.join("scroll_index.yaml");
    let contents = fs::read_to_string(idx).unwrap();
    assert!(contents.contains(file));
}
