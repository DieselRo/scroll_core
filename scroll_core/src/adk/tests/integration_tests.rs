#![allow(clippy::redundant_pattern_matching)]
#![allow(clippy::len_zero)]
// ==================================
// src/adk/tests/integration_tests.rs
// ==================================

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::pin::Pin;
    use std::sync::Arc;

    use async_trait::async_trait;
    use futures::{Stream, StreamExt};

    use crate::adk::agents::context::ToolContext;
    use crate::adk::agents::llm_agent::LlmAgent;
    use crate::adk::common::config::RunConfig;
    use crate::adk::common::error::AdkError;
    use crate::adk::common::types::{Content, FunctionCall, FunctionDeclaration, Part};

    use crate::adk::models::base_llm::BaseLlm;
    use crate::adk::models::llm_request::LlmRequest;
    use crate::adk::models::llm_response::LlmResponse;
    use crate::adk::runner::in_memory::new_in_memory_runner;
    use crate::adk::tools::base_tool::BaseTool;
    use crate::function_tool;

    // A mock LLM that simulates function calling
    struct FunctionCallingLlm;

    #[async_trait]
    impl BaseLlm for FunctionCallingLlm {
        fn model(&self) -> &str {
            "function-calling-model"
        }

        fn supported_models() -> Vec<String> {
            vec!["function-calling-model".to_string()]
        }

        async fn generate_content<'a>(
            &'a self,
            request: LlmRequest,
            _stream: bool,
        ) -> Result<Pin<Box<dyn Stream<Item = LlmResponse> + Send + 'a>>, AdkError> {
            // Check if there's a message about weather in the request
            let mut weather_request = false;

            for content in &request.contents {
                if let Some(role) = &content.role {
                    if role == "user" {
                        for part in &content.parts {
                            if let Some(text) = &part.text {
                                if text.to_lowercase().contains("weather") {
                                    weather_request = true;
                                    break;
                                }
                            }
                        }
                    }
                }

                if weather_request {
                    break;
                }
            }

            // If the user is asking about weather, call the weather tool
            if weather_request {
                let function_call = FunctionCall {
                    name: "get_weather".to_string(),
                    args: HashMap::from([("location".to_string(), serde_json::json!("New York"))]),
                };

                let content = Content {
                    role: Some("assistant".to_string()),
                    parts: vec![Part {
                        text: None,
                        inline_data: None,
                        function_call: Some(function_call),
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

            // Otherwise, return a normal message
            let content = Content {
                role: Some("assistant".to_string()),
                parts: vec![Part {
                    text: Some("I'm here to help. How can I assist you today?".to_string()),
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

    // A simple weather tool
    struct WeatherTool;

    #[async_trait]
    impl BaseTool for WeatherTool {
        fn name(&self) -> &str {
            "get_weather"
        }

        fn description(&self) -> &str {
            "Get the weather for a location"
        }

        fn declaration(&self) -> FunctionDeclaration {
            FunctionDeclaration {
                name: "get_weather".to_string(),
                description: "Get the weather for a location".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The location to get weather for"
                        }
                    },
                    "required": ["location"]
                }),
            }
        }

        async fn execute(
            &self,
            args: HashMap<String, serde_json::Value>,
            _context: ToolContext<'_>,
        ) -> Result<serde_json::Value, AdkError> {
            let location = args
                .get("location")
                .ok_or_else(|| AdkError::InvalidRequest("Missing location parameter".to_string()))?
                .as_str()
                .ok_or_else(|| AdkError::InvalidRequest("Location must be a string".to_string()))?;

            // Simulate getting weather data
            let temperature = if location.to_lowercase() == "new york" {
                75
            } else {
                70
            };

            Ok::<serde_json::Value, AdkError>(serde_json::json!({
                "temperature": temperature,
                "condition": "sunny",
                "location": location
            }))
        }
    }

    // A function tool to get the current time
    fn create_time_tool() -> impl BaseTool {
        function_tool!(
            "get_time",
            "Get the current time for a location",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "timezone": {
                        "type": "string",
                        "description": "The timezone to get the time for (e.g., 'UTC', 'America/New_York')"
                    }
                },
                "required": ["timezone"]
            }),
            |args: HashMap<String, serde_json::Value>, _context| async move {
                let timezone = args
                    .get("timezone")
                    .ok_or_else(|| {
                        AdkError::InvalidRequest("Missing timezone parameter".to_string())
                    })?
                    .as_str()
                    .ok_or_else(|| {
                        AdkError::InvalidRequest("Timezone must be a string".to_string())
                    })?;

                // Simulate getting time data
                let current_time = if timezone == "UTC" {
                    "2025-04-22T12:00:00Z"
                } else if timezone == "America/New_York" {
                    "2025-04-22T08:00:00-04:00"
                } else {
                    "Unknown timezone"
                };

                Ok::<serde_json::Value, AdkError>(serde_json::json!({
                    "time": current_time,
                    "timezone": timezone
                }))
            }
        )
    }

    #[tokio::test]
    async fn test_full_agent_tool_integration() {
        // Create tools
        let weather_tool = Arc::new(WeatherTool);
        let time_tool = Arc::new(create_time_tool());

        // Create an LLM
        let llm = Arc::new(FunctionCallingLlm);

        // Create an agent with tools
        let agent = Arc::new(LlmAgent::new(
            "assistant".to_string(),
            "A helpful assistant".to_string(),
            llm,
            "You are a helpful assistant that can provide weather information and tell the time.".to_string(),
        ).with_tools(vec![weather_tool, time_tool]));

        // Create a runner
        let runner = new_in_memory_runner("test-app".to_string(), agent);

        // Run with a weather query
        let user_id = "test-user";
        let session_id = "test-session";

        let content = Content {
            role: Some("user".to_string()),
            parts: vec![Part {
                text: Some("What's the weather like in New York?".to_string()),
                inline_data: None,
                function_call: None,
                function_response: None,
            }],
        };

        let mut event_stream = runner
            .run(user_id, session_id, Some(content), RunConfig::default())
            .await
            .unwrap();

        // Collect all events
        let mut events = Vec::new();
        while let Some(event) = event_stream.next().await {
            events.push(event);
        }

        // Check we got the expected events
        assert!(events.len() >= 1, "Expected at least one event");

        // Check the session was created with the events
        let session = runner
            .session_service
            .get_session("test-app", user_id, session_id, None)
            .await
            .unwrap();

        assert!(session.is_some());
        let session = session.unwrap();

        // Session should have at least the user's message
        assert!(
            session.events.len() >= 1,
            "Expected at least one event in the session"
        );

        // First event should be the user's message
        assert_eq!(session.events[0].author, "user");

        // Check function call was processed correctly
        let has_function_call = session.events.iter().any(|e| {
            if let Some(content) = &e.content {
                content.parts.iter().any(|p| p.function_call.is_some())
            } else {
                false
            }
        });

        let has_function_response = session.events.iter().any(|e| {
            if let Some(content) = &e.content {
                content.parts.iter().any(|p| p.function_response.is_some())
            } else {
                false
            }
        });

        // Either we should have a function call or we should have a function response
        assert!(
            has_function_call || has_function_response,
            "Expected either a function call or function response"
        );
    }

    #[tokio::test]
    async fn test_conversation_flow() {
        // Create an LLM
        let llm = Arc::new(FunctionCallingLlm);

        // Create an agent (no tools for this test)
        let agent = Arc::new(LlmAgent::new(
            "assistant".to_string(),
            "A helpful assistant".to_string(),
            llm,
            "You are a helpful assistant.".to_string(),
        ));

        // Create a runner
        let runner = new_in_memory_runner("test-app".to_string(), agent);

        // Run a series of messages in the same session
        let user_id = "test-user";
        let session_id = "test-session";

        // First message
        let content1 = Content {
            role: Some("user".to_string()),
            parts: vec![Part {
                text: Some("Hello, how are you?".to_string()),
                inline_data: None,
                function_call: None,
                function_response: None,
            }],
        };

        let mut event_stream = runner
            .run(user_id, session_id, Some(content1), RunConfig::default())
            .await
            .unwrap();

        // Consume events
        while let Some(_) = event_stream.next().await {}

        // Second message
        let content2 = Content {
            role: Some("user".to_string()),
            parts: vec![Part {
                text: Some("Can you help me with something?".to_string()),
                inline_data: None,
                function_call: None,
                function_response: None,
            }],
        };

        let mut event_stream = runner
            .run(user_id, session_id, Some(content2), RunConfig::default())
            .await
            .unwrap();

        // Consume events
        while let Some(_) = event_stream.next().await {}

        // Check the session has all the messages
        let session = runner
            .session_service
            .get_session("test-app", user_id, session_id, None)
            .await
            .unwrap();

        assert!(session.is_some());
        let session = session.unwrap();

        // Session should have 4 events (2 user messages, 2 assistant responses)
        assert_eq!(
            session.events.len(),
            4,
            "Expected 4 events, got {}",
            session.events.len()
        );

        // Check the events
        assert_eq!(session.events[0].author, "user");
        assert_eq!(session.events[1].author, "assistant");
        assert_eq!(session.events[2].author, "user");
        assert_eq!(session.events[3].author, "assistant");
    }
}
