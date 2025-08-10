#[test]
fn print_model_config_smoke() {
    use assert_cmd::Command;
    Command::cargo_bin("scroll_core")
        .unwrap()
        .arg("--print-model-config")
        .assert()
        .success()
        .stdout(predicates::str::contains("version").or(predicates::str::contains("default")).or(predicates::str::contains("constructs")));
}

