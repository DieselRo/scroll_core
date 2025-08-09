use scroll_core::invocation::llm::{
    mock::MockLLMClient, retry::RetryingClient, LLMClient, LLMError,
};
use std::sync::Arc;

#[tokio::test]
async fn retrying_client_retries_transient_then_succeeds() {
    // Prepare a mock that returns two transient errors, then success
    let script = vec![
        Ok("success".to_string()),
        Err(LLMError::Transient("flaky".into())),
        Err(LLMError::Timeout),
    ];
    // Pop order is LIFO in our mock; reverse to simulate sequence
    let mock = MockLLMClient::with_script(script.into_iter().rev().collect());
    let wrapped = RetryingClient::new(Arc::new(mock), 3, 100, 1);
    let out = wrapped.send("hello").await.unwrap();
    assert_eq!(out, "success");
}

#[tokio::test]
async fn retrying_client_stops_on_non_retryable() {
    let script = vec![Err(LLMError::Auth)];
    let mock = MockLLMClient::with_script(script);
    let wrapped = RetryingClient::new(Arc::new(mock), 3, 100, 1);
    let err = wrapped.send("hi").await.err().unwrap();
    assert!(matches!(err, LLMError::Auth));
}
