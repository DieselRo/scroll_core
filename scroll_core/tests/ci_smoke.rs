use std::process::Command;

#[test]
fn ci_commands_succeed_on_fresh_crate() {
    let temp = tempfile::tempdir().expect("create temp dir");
    let status = Command::new("cargo")
        .args(["new", "--lib", "dummy"])
        .current_dir(&temp)
        .status()
        .expect("cargo new failed");
    assert!(status.success());
    let crate_dir = temp.path().join("dummy");

    // Ensure rustfmt is installed so `cargo fmt` works
    let _ = Command::new("rustup")
        .args(["component", "add", "rustfmt"])
        .status();

    let status = Command::new("cargo")
        .args(["fmt", "--", "--check"])
        .current_dir(&crate_dir)
        .status()
        .expect("cargo fmt failed");
    assert!(status.success());

    let status = Command::new("cargo")
        .args(["clippy", "--", "-D", "warnings"])
        .current_dir(&crate_dir)
        .status()
        .expect("cargo clippy failed");
    assert!(status.success());

    let status = Command::new("cargo")
        .args(["test", "--all"])
        .current_dir(&crate_dir)
        .status()
        .expect("cargo test failed");
    assert!(status.success());
}
