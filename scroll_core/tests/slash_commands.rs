use assert_cmd::Command;
use predicates::str::contains;
use std::path::PathBuf;

#[test]
fn slash_help_lists_commands() {
    let archive = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../tests/e2e_scrolls");
    let mut cmd = Command::cargo_bin("scroll_core").unwrap();
    cmd.env("SCROLL_CORE_USE_MOCK", "1")
        .env("SCROLL_CI", "1")
        .env("PAGER", "cat")
        .env("SCROLL_CORE_ARCHIVE_DIR", archive)
        .args(["chat", "mythscribe", "--no-banner"])
        .write_stdin("/help\nexit\n")
        .assert()
        .success()
        .stdout(contains("scroll open"))
        .stdout(contains("scroll list"));
}
