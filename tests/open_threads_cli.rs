use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn open_threads_cli_ordering() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("open_threads.sqlite");
    let url = format!("sqlite://{}?mode=rwc", path.display());

    // create two threads
    let mut cmd = Command::cargo_bin("scroll_core").unwrap();
    cmd.env("DATABASE_URL", &url)
        .args([
            "open-threads",
            "--action",
            "create",
            "--title",
            "First",
            "--scroll",
            "scrolls/a.md",
        ])
        .assert()
        .success();

    let mut cmd2 = Command::cargo_bin("scroll_core").unwrap();
    cmd2.env("DATABASE_URL", &url)
        .args([
            "open-threads",
            "--action",
            "create",
            "--title",
            "Second",
            "--scroll",
            "scrolls/a.md",
        ])
        .assert()
        .success();

    // list with deterministic order (created_at asc then id asc)
    let output = Command::cargo_bin("scroll_core")
        .unwrap()
        .env("DATABASE_URL", &url)
        .args(["open-threads", "--action", "list", "--scroll", "scrolls/a.md"]) // no status filter
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let s = String::from_utf8_lossy(&output);
    let mut lines: Vec<&str> = s.lines().collect();
    // Expect at least 2 lines and ordered by creation
    assert!(lines.len() >= 2);
    // Extract titles from printed lines (id | status | scroll | title | created_at)
    let title1 = lines[0].split(" | ").nth(3).unwrap_or("");
    let title2 = lines[1].split(" | ").nth(3).unwrap_or("");
    assert_eq!(title1, "First");
    assert_eq!(title2, "Second");
}

