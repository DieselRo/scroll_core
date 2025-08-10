// ==================================
// invocation/llm/mod.rs
// Provider-agnostic LLM client layer
// ==================================

// top-level Arc import not needed; submodules import Arc directly

use async_trait::async_trait;
use thiserror::Error;

pub mod mock;
pub mod openai;

/// Lightweight factory to build an `Arc<dyn LLMClient>` from env.
pub mod factory {
    use super::{
        mock::MockLLMClient, openai::OpenAIClient, retry::RetryingClient, LLMClient, LLMError,
    };
    use crate::models::model_registry::{ModelSpec, Provider};
    use std::sync::Arc;

    /// Creates an LLM client from environment configuration.
    ///
    /// Env vars:
    /// - SC_LLM_PROVIDER: "openai" | "mock" (default: openai; in tests default: mock)
    /// - SC_LLM_MAX_RETRIES: usize (default: 2)
    /// - SC_LLM_ATTEMPT_TIMEOUT_MS: u64 (default: 10000)
    /// - SC_LLM_BASE_BACKOFF_MS: u64 (default: 200)
    pub fn from_env() -> Result<Arc<dyn LLMClient>, LLMError> {
        let provider = std::env::var("SC_LLM_PROVIDER")
            .ok()
            .unwrap_or_else(|| {
                if cfg!(test) {
                    "mock".into()
                } else {
                    "openai".into()
                }
            })
            .to_lowercase();

        let max_retries = std::env::var("SC_LLM_MAX_RETRIES")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(2);
        let attempt_timeout_ms = std::env::var("SC_LLM_ATTEMPT_TIMEOUT_MS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(10_000);
        let base_backoff_ms = std::env::var("SC_LLM_BASE_BACKOFF_MS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(200);

        let client: Arc<dyn LLMClient> = match provider.as_str() {
            "mock" => Arc::new(MockLLMClient::default()),
            "openai" => Arc::new(OpenAIClient::new_from_env()?),
            _ => Arc::new(OpenAIClient::new_from_env()?),
        };

        let wrapped = RetryingClient::new(client, max_retries, attempt_timeout_ms, base_backoff_ms);
        Ok(Arc::new(wrapped))
    }

    /// Creates an LLM client from a resolved `ModelSpec`.
    pub fn from_spec(spec: &ModelSpec) -> Result<Arc<dyn LLMClient>, LLMError> {
        let max_retries = std::env::var("SC_LLM_MAX_RETRIES")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(2);
        let attempt_timeout_ms = std::env::var("SC_LLM_ATTEMPT_TIMEOUT_MS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(10_000);
        let base_backoff_ms = std::env::var("SC_LLM_BASE_BACKOFF_MS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(200);

        let client: Arc<dyn LLMClient> = match spec.provider {
            Provider::Mock | Provider::Local => Arc::new(MockLLMClient::default()),
            Provider::OpenAI => {
                let endpoint = std::env::var("SC_LLM_ENDPOINT")
                    .unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".to_string());
                let api_key = std::env::var("OPENAI_API_KEY")
                    .map_err(|_| LLMError::Config("OPENAI_API_KEY not set".into()))?;
                let timeout_ms = std::env::var("SC_LLM_GLOBAL_TIMEOUT_MS")
                    .ok()
                    .and_then(|s| s.parse::<u64>().ok());
                Arc::new(OpenAIClient::new(
                    api_key,
                    spec.model.clone(),
                    endpoint,
                    timeout_ms,
                )?)
            }
            Provider::Anthropic => {
                // For now, route unsupported providers to a Config error
                return Err(LLMError::Unsupported(
                    "Anthropic provider not implemented".into(),
                ));
            }
        };

        let wrapped = RetryingClient::new(client, max_retries, attempt_timeout_ms, base_backoff_ms);
        Ok(Arc::new(wrapped))
    }
}

/// Retry/backoff decorator for any LLMClient implementation.
pub mod retry {
    use super::{LLMClient, LLMError};
    use async_trait::async_trait;
    use std::sync::Arc;
    use tokio::time::{sleep, timeout, Duration};

    pub struct RetryingClient {
        inner: Arc<dyn LLMClient + Send + Sync>,
        max_retries: usize,
        attempt_timeout: Duration,
        base_backoff: Duration,
    }

    impl RetryingClient {
        pub fn new(
            inner: Arc<dyn LLMClient + Send + Sync>,
            max_retries: usize,
            attempt_timeout_ms: u64,
            base_backoff_ms: u64,
        ) -> Self {
            Self {
                inner,
                max_retries,
                attempt_timeout: Duration::from_millis(attempt_timeout_ms),
                base_backoff: Duration::from_millis(base_backoff_ms),
            }
        }

        fn should_retry(err: &LLMError) -> bool {
            matches!(
                err,
                LLMError::Timeout
                    | LLMError::Transient(_)
                    | LLMError::Network(_)
                    | LLMError::Server(_)
                    | LLMError::RateLimit
            )
        }

        fn backoff_for(&self, attempt: usize) -> Duration {
            // Exponential backoff without jitter to avoid extra deps; 200ms * 2^attempt, capped.
            let pow = 1u64 << (attempt.min(6) as u32); // cap growth at 2^6
            let ms = self.base_backoff.as_millis() as u64 * pow;
            Duration::from_millis(ms.min(5_000))
        }
    }

    #[async_trait]
    impl LLMClient for RetryingClient {
        async fn send(&self, prompt: &str) -> Result<String, LLMError> {
            let mut last_err: Option<LLMError> = None;
            for attempt in 0..=self.max_retries {
                let fut = self.inner.send(prompt);
                match timeout(self.attempt_timeout, fut).await {
                    Ok(Ok(resp)) => return Ok(resp),
                    Ok(Err(e)) => {
                        if attempt >= self.max_retries || !Self::should_retry(&e) {
                            return Err(e);
                        }
                        last_err = Some(e);
                    }
                    Err(_) => {
                        let e = LLMError::Timeout;
                        if attempt >= self.max_retries || !Self::should_retry(&e) {
                            return Err(e);
                        }
                        last_err = Some(e);
                    }
                }

                sleep(self.backoff_for(attempt)).await;
            }
            Err(last_err.unwrap_or(LLMError::Other("unknown failure".into())))
        }

        async fn send_stream(&self, _prompt: &str) -> Result<super::TokenStream, LLMError> {
            Err(LLMError::Unsupported(
                "streaming not supported by retry wrapper".into(),
            ))
        }
    }
}

#[derive(Debug, Error, Clone)]
pub enum LLMError {
    #[error("configuration error: {0}")]
    Config(String),
    #[error("authentication error")]
    Auth,
    #[error("rate limited")]
    RateLimit,
    #[error("request timed out")]
    Timeout,
    #[error("transient error: {0}")]
    Transient(String),
    #[error("network error: {0}")]
    Network(String),
    #[error("server error: {0}")]
    Server(String),
    #[error("unsupported: {0}")]
    Unsupported(String),
    #[error("other: {0}")]
    Other(String),
}

pub type TokenStream = Box<dyn futures::Stream<Item = Result<String, LLMError>> + Send + Unpin>;

#[async_trait]
pub trait LLMClient: Send + Sync {
    async fn send(&self, prompt: &str) -> Result<String, LLMError>;

    async fn send_stream(&self, _prompt: &str) -> Result<TokenStream, LLMError> {
        Err(LLMError::Unsupported(
            "streaming not implemented for this client".into(),
        ))
    }
}

/// Utilities for bridging async LLM calls from sync code.
pub mod util {
    use super::{LLMClient, LLMError};
    use tokio::runtime::{Builder, Handle};

    /// Executes `client.send(prompt)` from synchronous contexts.
    pub fn send_blocking(client: &dyn LLMClient, prompt: &str) -> Result<String, LLMError> {
        if let Ok(handle) = Handle::try_current() {
            handle.block_on(client.send(prompt))
        } else {
            let rt = Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| LLMError::Other(e.to_string()))?;
            rt.block_on(client.send(prompt))
        }
    }
}
