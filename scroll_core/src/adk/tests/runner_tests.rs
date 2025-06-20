// ==================================
// src/adk/tests/runner_tests.rs
// ==================================

#[cfg(test)]
mod tests {

    use std::pin::Pin;
    use std::sync::Arc;

    use async_trait::async_trait;
    use futures::{Stream, StreamExt};

    use crate::adk::agents::base_agent::BaseAgent;
    use crate::adk::agents::context::InvocationContext;
    use crate::adk::common::config::RunConfig;
    use crate::adk::common::error::AdkError;
    use crate::adk::common::types::{Content, Part};
    use crate::adk::events::event::Event;
    use crate::adk::models::base_llm::BaseLlm;
    use crate::adk::models::llm_request::LlmRequest;
    use crate::adk::models::llm_response::LlmResponse;
    use crate::adk::runner::in_memory::new_in_memory_runner;

    // A mock LLM that always returns a fixed response
    #[allow(dead_code)]
    struct MockLlm;

    #[async_trait]
    impl BaseLlm for MockLlm {
        fn model(&self) -> &str {
            "mock-model"
        }

        fn supported_models() -> Vec<String> {
            vec!["mock-model".to_string()]
        }

        async fn generate_content<'a>(
            &'a self,
            _request: LlmRequest,
            _stream: bool,
        ) -> Result<Pin<Box<dyn Stream<Item = LlmResponse> + Send + 'a>>, AdkError> {
            let content = Content {
                role: Some("assistant".to_string()),
                parts: vec![Part {
                    text: Some("This is a mock response".to_string()),
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

    // A simple echo agent that just echoes the user's message
    struct EchoAgent;

    #[async_trait]
    impl BaseAgent for EchoAgent {
        fn name(&self) -> &str {
            "echo-agent"
        }

        fn description(&self) -> &str {
            "An agent that echoes the user's message"
        }

        async fn run_async<'a>(
            &'a self,
            context: InvocationContext,
        ) -> Result<Pin<Box<dyn Stream<Item = Event> + Send + 'a>>, AdkError> {
            let user_content = context.user_content.clone();

            let response_content = if let Some(content) = user_content {
                let mut parts = Vec::new();

                for part in content.parts {
                    if let Some(text) = part.text {
                        parts.push(Part {
                            text: Some(format!("Echo: {}", text)),
                            inline_data: None,
                            function_call: None,
                            function_response: None,
                        });
                    }
                }

                Content {
                    role: Some("assistant".to_string()),
                    parts,
                }
            } else {
                Content {
                    role: Some("assistant".to_string()),
                    parts: vec![Part {
                        text: Some("Nothing to echo".to_string()),
                        inline_data: None,
                        function_call: None,
                        function_response: None,
                    }],
                }
            };

            let event = Event::new_agent_event(
                &context.invocation_id,
                self.name(),
                response_content,
                false,
            );

            Ok(Box::pin(futures::stream::once(futures::future::ready(
                event,
            ))))
        }
    }

    #[tokio::test]
    async fn test_runner_create_session() {
        // Create a runner
        let agent = Arc::new(EchoAgent);
        let runner = new_in_memory_runner("test-app".to_string(), agent);

        // Run with a new session ID
        let user_id = "test-user";
        let session_id = "test-session";

        let content = Content {
            role: Some("user".to_string()),
            parts: vec![Part {
                text: Some("Hello, world!".to_string()),
                inline_data: None,
                function_call: None,
                function_response: None,
            }],
        };

        let mut event_stream = runner
            .run(user_id, session_id, Some(content), RunConfig::default())
            .await
            .unwrap();

        // Check we get a response
        let event = event_stream.next().await.unwrap();
        assert_eq!(event.author, "echo-agent");

        // Check the session was created
        let session = runner
            .session_service
            .get_session("test-app", user_id, session_id, None)
            .await
            .unwrap();

        assert!(session.is_some());
        let session = session.unwrap();
        assert_eq!(session.id, session_id);

        // Session should have both the user's message and the agent's response
        assert_eq!(session.events.len(), 2);
    }

    #[tokio::test]
    async fn test_runner_use_existing_session() {
        // Create a runner
        let agent = Arc::new(EchoAgent);
        let runner = new_in_memory_runner("test-app".to_string(), agent);

        // Create a session first
        let user_id = "test-user";
        let session_id = "test-session";

        runner
            .session_service
            .create_session("test-app", user_id, None, Some(session_id))
            .await
            .unwrap();

        // Run with the existing session
        let content = Content {
            role: Some("user".to_string()),
            parts: vec![Part {
                text: Some("Hello, world!".to_string()),
                inline_data: None,
                function_call: None,
                function_response: None,
            }],
        };

        let mut event_stream = runner
            .run(user_id, session_id, Some(content), RunConfig::default())
            .await
            .unwrap();

        // Check we get a response
        let event = event_stream.next().await.unwrap();
        assert_eq!(event.author, "echo-agent");

        // Check the session was used
        let session = runner
            .session_service
            .get_session("test-app", user_id, session_id, None)
            .await
            .unwrap();

        assert!(session.is_some());
        let session = session.unwrap();

        // Session should have both the user's message and the agent's response
        assert_eq!(session.events.len(), 2);
    }

    #[tokio::test]
    async fn test_runner_run_without_message() {
        // Create a runner
        let agent = Arc::new(EchoAgent);
        let runner = new_in_memory_runner("test-app".to_string(), agent);

        // Run without a message
        let user_id = "test-user";
        let session_id = "test-session";

        let mut event_stream = runner
            .run(user_id, session_id, None, RunConfig::default())
            .await
            .unwrap();

        // Check we get a response
        let event = event_stream.next().await.unwrap();
        assert_eq!(event.author, "echo-agent");

        // The response should be "Nothing to echo"
        let content = event.content.unwrap();
        assert_eq!(content.parts[0].text.as_ref().unwrap(), "Nothing to echo");
    }

    #[tokio::test]
    async fn test_runner_close_session() {
        // Create a runner
        let agent = Arc::new(EchoAgent);
        let runner = new_in_memory_runner("test-app".to_string(), agent);

        // Create a session
        let user_id = "test-user";
        let session_id = "test-session";

        let mut session = runner
            .session_service
            .create_session("test-app", user_id, None, Some(session_id))
            .await
            .unwrap();

        // Close the session
        runner.close_session(&mut session).await.unwrap();

        // Check the session was closed
        let session = runner
            .session_service
            .get_session("test-app", user_id, session_id, None)
            .await
            .unwrap();

        assert!(session.is_some());
        let session = session.unwrap();
        assert_eq!(
            session.status,
            crate::adk::sessions::session::SessionStatus::Closed
        );
    }
}
