#[test]
fn context_cli_exports_json() {
    use assert_cmd::Command;
    use serde_json::Value;
    let output = Command::cargo_bin("scroll_core")
        .unwrap()
        .args(["context", "--limit", "1", "--details", "--export", "json"])
        .output()
        .expect("run");
    assert!(output.status.success());
    let s = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str::<Value>(&s).expect("valid json");
}
