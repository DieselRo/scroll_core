use super::{LLMClient, LLMError};
use async_trait::async_trait;
use reqwest::StatusCode;

#[derive(Clone)]
pub struct OpenAIClient {
    api_key: String,
    model: String,
    endpoint: String,
    http: reqwest::Client,
}

impl OpenAIClient {
    pub fn new_from_env() -> Result<Self, LLMError> {
        dotenvy::dotenv().ok();
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| LLMError::Config("OPENAI_API_KEY not set".into()))?;
        let model = std::env::var("SC_LLM_MODEL").unwrap_or_else(|_| "gpt-4o".to_string());
        let endpoint = std::env::var("SC_LLM_ENDPOINT")
            .unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".to_string());
        // Global timeout; attempt-level timeouts are handled by the retry wrapper
        let timeout_ms = std::env::var("SC_LLM_GLOBAL_TIMEOUT_MS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(15_000);
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(timeout_ms))
            .build()
            .map_err(|e| LLMError::Other(format!("http client build error: {e}")))?;
        Ok(Self {
            api_key,
            model,
            endpoint,
            http,
        })
    }

    /// Construct a client with explicit config (useful in tests).
    pub fn new(
        api_key: String,
        model: String,
        endpoint: String,
        timeout_ms: Option<u64>,
    ) -> Result<Self, LLMError> {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(
                timeout_ms.unwrap_or(15_000),
            ))
            .build()
            .map_err(|e| LLMError::Other(format!("http client build error: {e}")))?;
        Ok(Self {
            api_key,
            model,
            endpoint,
            http,
        })
    }
}

#[async_trait]
impl LLMClient for OpenAIClient {
    async fn send(&self, prompt: &str) -> Result<String, LLMError> {
        let body = serde_json::json!({
            "model": self.model,
            "messages": [
                {"role": "system", "content": prompt}
            ],
            "max_tokens": 750
        });

        let resp = self
            .http
            .post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                if e.is_connect() || e.is_request() {
                    LLMError::Network(e.to_string())
                } else if e.is_timeout() {
                    LLMError::Timeout
                } else {
                    LLMError::Other(e.to_string())
                }
            })?;

        let status = resp.status();
        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| LLMError::Other(format!("JSON parse error: {e}")))?;

        match status {
            StatusCode::OK => {
                let text = json["choices"][0]["message"]["content"]
                    .as_str()
                    .ok_or_else(|| LLMError::Other("malformed response".into()))?
                    .to_string();
                Ok(text)
            }
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Err(LLMError::Auth),
            StatusCode::TOO_MANY_REQUESTS => Err(LLMError::RateLimit),
            s if s.is_server_error() => Err(LLMError::Server(format!("{s}"))),
            _ => Err(LLMError::Other(format!("unexpected status: {status}"))),
        }
    }
}
