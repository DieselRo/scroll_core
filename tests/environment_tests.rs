use scroll_core::validate_scroll_environment;

#[test]
fn test_missing_openai_key() {
    std::env::remove_var("OPENAI_API_KEY");
    assert!(!validate_scroll_environment());
}
