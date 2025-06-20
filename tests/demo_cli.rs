#[test]
fn demo_runs_end_to_end() {
    use assert_cmd::Command;
    Command::cargo_bin("scroll_core")
        .unwrap()
        .args(&["--demo", "examples/multi_agent.yaml"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Assistant replied"));
}
