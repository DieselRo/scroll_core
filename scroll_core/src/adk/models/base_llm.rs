// ==================================
// src/adk/models/base_llm.rs
// ==================================

use async_trait::async_trait;
use futures::stream::Stream;
use std::pin::Pin;

use crate::adk::common::error::AdkError;
use crate::adk::models::llm_request::LlmRequest;
use crate::adk::models::llm_response::LlmResponse;

/// Base trait for LLM implementations
#[async_trait]
pub trait BaseLlm: Send + Sync {
    /// Get the model name/identifier
    fn model(&self) -> &str;
    
    /// Get the supported models
    fn supported_models() -> Vec<String> where Self: Sized;
    
    /// Generate content from the LLM
    async fn generate_content<'a>(
        &'a self,
        request: LlmRequest,
        stream: bool,
    ) -> Result<Pin<Box<dyn Stream<Item = LlmResponse> + Send + 'a>>, AdkError>;
}