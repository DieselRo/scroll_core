// ==================================
// src/adk/tests/agent_tests.rs
// ==================================

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::pin::Pin;
    use std::sync::Arc;

    use async_trait::async_trait;
    use futures::{Stream, StreamExt};

    use crate::adk::agents::base_agent::BaseAgent;
    use crate::adk::agents::context::InvocationContext;
    use crate::adk::agents::llm_agent::LlmAgent;
    use crate::adk::common::config::RunConfig;
    use crate::adk::common::error::AdkError;
    use crate::adk::common::types::{Content, Part};
    use crate::adk::events::event::Event;
    use crate::adk::events::event_actions::{EventActions, TransferToAgentAction};
    use crate::adk::models::base_llm::BaseLlm;
    use crate::adk::models::llm_request::LlmRequest;
    use crate::adk::models::llm_response::LlmResponse;
    use crate::adk::sessions::base_session_service::BaseSessionService;
    use crate::adk::sessions::in_memory_service::InMemorySessionService;

    // A mock LLM implementation
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

    // A mock LLM that can transfer to another agent
    #[allow(dead_code)]
    struct TransferLlm;

    #[async_trait]
    impl BaseLlm for TransferLlm {
        fn model(&self) -> &str {
            "transfer-model"
        }

        fn supported_models() -> Vec<String> {
            vec!["transfer-model".to_string()]
        }

        async fn generate_content<'a>(
            &'a self,
            _request: LlmRequest,
            _stream: bool,
        ) -> Result<Pin<Box<dyn Stream<Item = LlmResponse> + Send + 'a>>, AdkError> {
            let mut actions = EventActions::default();
            actions.transfer_to_agent = Some(TransferToAgentAction {
                agent_name: "sub-agent".to_string(),
                context: None,
            });

            let content = Content {
                role: Some("assistant".to_string()),
                parts: vec![Part {
                    text: Some("I'll transfer you to another agent".to_string()),
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

    // A simple subagent
    struct SubAgent;

    #[async_trait]
    impl BaseAgent for SubAgent {
        fn name(&self) -> &str {
            "sub-agent"
        }

        fn description(&self) -> &str {
            "A sub agent"
        }

        async fn run_async<'a>(
            &'a self,
            context: InvocationContext,
        ) -> Result<Pin<Box<dyn Stream<Item = Event> + Send + 'a>>, AdkError> {
            let content = Content {
                role: Some("assistant".to_string()),
                parts: vec![Part {
                    text: Some("Hello from the sub agent".to_string()),
                    inline_data: None,
                    function_call: None,
                    function_response: None,
                }],
            };

            let event = Event::new_agent_event(&context.invocation_id, self.name(), content, false);

            Ok(Box::pin(futures::stream::once(futures::future::ready(
                event,
            ))))
        }
    }

    #[tokio::test]
    async fn test_llm_agent_basic() {
        // Create an LLM
        let llm = Arc::new(MockLlm);

        // Create an agent
        let agent = LlmAgent::new(
            "test-agent".to_string(),
            "A test agent".to_string(),
            llm,
            "You are a test agent.".to_string(),
        );

        // Create a session service
        let session_service = Arc::new(InMemorySessionService::new());

        // Create a session
        let session = session_service
            .create_session("test-app", "test-user", None, Some("test-session"))
            .await
            .unwrap();

        // Create an invocation context
        let context = InvocationContext {
            artifact_service: None,
            session_service: session_service.clone(),
            memory_service: None,
            invocation_id: "test-invocation".to_string(),
            branch: None,
            agent: Arc::new(agent.clone()),
            user_content: Some(Content {
                role: Some("user".to_string()),
                parts: vec![Part {
                    text: Some("Hello".to_string()),
                    inline_data: None,
                    function_call: None,
                    function_response: None,
                }],
            }),
            session,
            end_invocation: false,
            run_config: RunConfig::default(),
            active_streaming_tools: HashMap::new(),
        };

        // Run the agent
        let agent_arc = Arc::new(agent);
        let mut event_stream = agent_arc.run_async(context).await.unwrap();

        // Check we get a response
        let event = event_stream.next().await.unwrap();
        assert_eq!(event.author, "test-agent");

        // Check the content
        assert!(event.content.is_some());
        let content = event.content.unwrap();
        assert_eq!(
            content.parts[0].text.as_ref().unwrap(),
            "This is a mock response"
        );
    }

    #[tokio::test]
    async fn test_llm_agent_with_sub_agents() {
        // Create an LLM
        let llm = Arc::new(MockLlm);

        // Create a sub agent
        let sub_agent = Arc::new(SubAgent);

        // Create an agent with a sub agent
        let agent = LlmAgent::new(
            "test-agent".to_string(),
            "A test agent".to_string(),
            llm,
            "You are a test agent.".to_string(),
        )
        .with_sub_agents(vec![sub_agent.clone()]);

        // Check agent name and description
        assert_eq!(agent.name(), "test-agent");
        assert_eq!(agent.description(), "A test agent");

        // Check sub agent access
        let found_agent = agent.find_sub_agent("sub-agent");
        assert!(found_agent.is_some());
        assert_eq!(found_agent.unwrap().name(), "sub-agent");

        // Check non-existent agent
        let not_found = agent.find_sub_agent("nonexistent");
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_agent_streaming() {
        // Create a streaming LLM
        struct StreamingLlm;

        #[async_trait]
        impl BaseLlm for StreamingLlm {
            fn model(&self) -> &str {
                "streaming-model"
            }

            fn supported_models() -> Vec<String> {
                vec!["streaming-model".to_string()]
            }

            async fn generate_content<'a>(
                &'a self,
                _request: LlmRequest,
                stream: bool,
            ) -> Result<Pin<Box<dyn Stream<Item = LlmResponse> + Send + 'a>>, AdkError>
            {
                // Only stream if requested
                if !stream {
                    let content = Content {
                        role: Some("assistant".to_string()),
                        parts: vec![Part {
                            text: Some("Non-streaming response".to_string()),
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

                    return Ok(Box::pin(futures::stream::once(futures::future::ready(
                        response,
                    ))));
                }

                // Create three streaming responses
                let responses = vec![
                    LlmResponse {
                        content: Some(Content {
                            role: Some("assistant".to_string()),
                            parts: vec![Part {
                                text: Some("Streaming ".to_string()),
                                inline_data: None,
                                function_call: None,
                                function_response: None,
                            }],
                        }),
                        partial: Some(true),
                        error_code: None,
                        error_message: None,
                    },
                    LlmResponse {
                        content: Some(Content {
                            role: Some("assistant".to_string()),
                            parts: vec![Part {
                                text: Some("Streaming response ".to_string()),
                                inline_data: None,
                                function_call: None,
                                function_response: None,
                            }],
                        }),
                        partial: Some(true),
                        error_code: None,
                        error_message: None,
                    },
                    LlmResponse {
                        content: Some(Content {
                            role: Some("assistant".to_string()),
                            parts: vec![Part {
                                text: Some("Streaming response complete".to_string()),
                                inline_data: None,
                                function_call: None,
                                function_response: None,
                            }],
                        }),
                        partial: None,
                        error_code: None,
                        error_message: None,
                    },
                ];

                Ok(Box::pin(futures::stream::iter(responses)))
            }
        }

        // Create an agent
        let llm = Arc::new(StreamingLlm);
        let agent = Arc::new(LlmAgent::new(
            "streaming-agent".to_string(),
            "A streaming agent".to_string(),
            llm,
            "You are a streaming agent.".to_string(),
        ));

        // Create a session service
        let session_service = Arc::new(InMemorySessionService::new());

        // Create a session
        let session = session_service
            .create_session("test-app", "test-user", None, Some("test-session"))
            .await
            .unwrap();

        // Create an invocation context with streaming
        let mut run_config = RunConfig::default();
        run_config.streaming_mode = Some(crate::adk::common::config::StreamingMode::Streaming);

        let context = InvocationContext {
            artifact_service: None,
            session_service: session_service.clone(),
            memory_service: None,
            invocation_id: "test-invocation".to_string(),
            branch: None,
            agent: agent.clone(),
            user_content: Some(Content {
                role: Some("user".to_string()),
                parts: vec![Part {
                    text: Some("Hello".to_string()),
                    inline_data: None,
                    function_call: None,
                    function_response: None,
                }],
            }),
            session,
            end_invocation: false,
            run_config,
            active_streaming_tools: HashMap::new(),
        };

        // Run the agent
        let mut event_stream = agent.run_async(context).await.unwrap();

        // Collect all events
        let mut events = Vec::new();
        while let Some(event) = event_stream.next().await {
            events.push(event);
        }

        // We should have received 3 events
        assert_eq!(events.len(), 3);

        // All events should be from the streaming agent
        for event in &events {
            assert_eq!(event.author, "streaming-agent");
        }

        // Check partial flags
        assert_eq!(events[0].partial, Some(true));
        assert_eq!(events[1].partial, Some(true));
        assert_eq!(events[2].partial, Some(false));

        // Check content
        let content0 = events[0].content.as_ref().unwrap();
        let content1 = events[1].content.as_ref().unwrap();
        let content2 = events[2].content.as_ref().unwrap();

        assert_eq!(content0.parts[0].text.as_ref().unwrap(), "Streaming ");
        assert_eq!(
            content1.parts[0].text.as_ref().unwrap(),
            "Streaming response "
        );
        assert_eq!(
            content2.parts[0].text.as_ref().unwrap(),
            "Streaming response complete"
        );
    }
}
