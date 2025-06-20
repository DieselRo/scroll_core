// ==================================
// src/bin/adk_example.rs
// ==================================

use scroll_core::adk::agents::llm_agent::LlmAgent;
use scroll_core::adk::common::config::RunConfig;
use scroll_core::adk::common::error::AdkError;
use scroll_core::adk::common::types::{Content, Part};
use scroll_core::adk::models::base_llm::BaseLlm;
use scroll_core::adk::models::llm_request::LlmRequest;
use scroll_core::adk::models::llm_response::LlmResponse;
use scroll_core::adk::runner::in_memory::new_in_memory_runner;

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use std::pin::Pin;
use std::sync::Arc;
use uuid::Uuid;

/// Simple mock LLM implementation
struct MockLlm;

#[async_trait]
impl BaseLlm for MockLlm {
    fn model(&self) -> &str {
        "mock-llm-1.0"
    }

    fn supported_models() -> Vec<String> {
        vec!["mock-llm-1.0".to_string()]
    }

    async fn generate_content<'a>(
        &'a self,
        _request: LlmRequest,
        _stream: bool,
    ) -> Result<Pin<Box<dyn Stream<Item = LlmResponse> + Send + 'a>>, AdkError> {
        let content = Content {
            role: Some("assistant".to_string()),
            parts: vec![Part {
                text: Some(
                    "Hello, I'm a mock LLM! I received your message and I'm here to help."
                        .to_string(),
                ),
                inline_data: None,
                function_call: None,
                function_response: None,
            }],
        };

        let response = LlmResponse {
            content: Some(content),
            partial: None,
            error_code: None,
            error_message: None,
        };

        Ok(Box::pin(futures::stream::once(futures::future::ready(
            response,
        ))))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a mock LLM model
    let model = Arc::new(MockLlm);

    // Create an agent
    let agent = Arc::new(LlmAgent::new(
        "test-agent".to_string(),
        "A test agent".to_string(),
        model,
        "You are a helpful assistant.".to_string(),
    ));

    // Create an in-memory runner
    let runner = new_in_memory_runner("test-app".to_string(), agent);

    // Create a session ID
    let user_id = "test-user";
    let session_id = Uuid::new_v4().to_string();

    // Create a user message
    let user_message = Content {
        role: Some("user".to_string()),
        parts: vec![Part {
            text: Some("Hello, this is a test message.".to_string()),
            inline_data: None,
            function_call: None,
            function_response: None,
        }],
    };

    println!("Starting test session with ID: {}", session_id);
    println!("Sending user message: Hello, this is a test message.");

    // Run the agent
    let mut event_stream = runner
        .run(
            user_id,
            &session_id,
            Some(user_message),
            RunConfig::default(),
        )
        .await?;

    // Process events
    println!("\nAgent responses:");
    while let Some(event) = event_stream.next().await {
        if let Some(content) = &event.content {
            if let Some(part) = content.parts.iter().find(|p| p.text.is_some()) {
                println!("  {}: {}", event.author, part.text.as_ref().unwrap());
            }
        }
    }

    // Get the final session
    let runner_service = &runner.session_service;
    let session = runner_service
        .get_session("test-app", user_id, &session_id, None)
        .await?;

    if let Some(session) = session {
        println!("\nSession after conversation:");
        println!("  ID: {}", session.id);
        println!("  Events: {}", session.events.len());
        println!("  Status: {:?}", session.status);
    }

    Ok(())
}
