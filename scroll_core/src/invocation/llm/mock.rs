use super::{LLMClient, LLMError};
use async_trait::async_trait;
use std::sync::Mutex;

#[derive(Default)]
pub struct MockLLMClient {
    // If provided, will pop a result per call; otherwise returns a static message.
    scripted: Mutex<Vec<Result<String, LLMError>>>,
}

impl MockLLMClient {
    pub fn with_script(script: Vec<Result<String, LLMError>>) -> Self {
        Self {
            scripted: Mutex::new(script),
        }
    }
}

#[async_trait]
impl LLMClient for MockLLMClient {
    async fn send(&self, _prompt: &str) -> Result<String, LLMError> {
        if let Some(res) = self.scripted.lock().unwrap().pop() {
            return res;
        }
        Ok("[mock] The Archive hums softly.".into())
    }
}
