// ==================================
// src/adk/tests/tool_tests.rs
// ==================================

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::pin::Pin;
    
    use crate::adk::agents::context::{InvocationContext, ToolContext};
    use crate::adk::common::config::RunConfig;
    use crate::adk::common::error::AdkError;
    use crate::adk::common::types::{Content, FunctionDeclaration, Part};
    use crate::adk::sessions::in_memory_service::InMemorySessionService;
    use crate::adk::sessions::base_session_service::BaseSessionService;
    use crate::adk::sessions::session::Session;
    use crate::adk::tools::base_tool::BaseTool;
    use crate::adk::tools::function_tool::FunctionTool;
    use crate::function_tool;
    
    use async_trait::async_trait;
    
    // A simple test tool
    struct TestTool;
    
    #[async_trait]
    impl BaseTool for TestTool {
        fn name(&self) -> &str {
            "test-tool"
        }
        
        fn description(&self) -> &str {
            "A test tool"
        }
        
        fn declaration(&self) -> FunctionDeclaration {
            FunctionDeclaration {
                name: "test-tool".to_string(),
                description: "A test tool".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "input": {
                            "type": "string",
                            "description": "Input to the tool"
                        }
                    },
                    "required": ["input"]
                }),
            }
        }
        
        async fn execute(
            &self,
            args: HashMap<String, serde_json::Value>,
            _context: ToolContext<'_>,
        ) -> Result<serde_json::Value, AdkError> {
            let input = args.get("input")
                .ok_or_else(|| AdkError::InvalidRequest("Missing input parameter".to_string()))?
                .as_str()
                .ok_or_else(|| AdkError::InvalidRequest("Input must be a string".to_string()))?;
            
            Ok(serde_json::json!({
                "result": format!("Processed: {}", input)
            }))
        }
    }
    
    #[tokio::test]
    async fn test_basic_tool() {
        // Create a tool
        let tool = TestTool;
        
        // Check declaration
        let decl = tool.declaration();
        assert_eq!(decl.name, "test-tool");
        assert_eq!(decl.description, "A test tool");
        
        // Create invocation context
        let session_service = Arc::new(InMemorySessionService::new());
        let session = session_service.create_session(
            "test-app",
            "test-user",
            None,
            Some("test-session"),
        ).await.unwrap();
        
        let context = InvocationContext {
            artifact_service: None,
            session_service: session_service.clone(),
            memory_service: None,
            invocation_id: "test-invocation".to_string(),
            branch: None,
            agent: Arc::new(TestAgent),
            user_content: None,
            session,
            end_invocation: false,
            run_config: RunConfig::default(),
            active_streaming_tools: HashMap::new(),
        };
        
        let tool_context = ToolContext {
            invocation_context: &context,
            execution_id: "test-execution".to_string(),
        };
        
        // Execute the tool
        let args = HashMap::from([
            ("input".to_string(), serde_json::json!("test input")),
        ]);
        
        let result = tool.execute(args, tool_context).await.unwrap();
        
        // Check result
        assert_eq!(result["result"], "Processed: test input");
    }
    
    #[tokio::test]
    async fn test_function_tool() {
        // Create a function tool
        let tool = function_tool!(
            "function-tool",
            "A function-based tool",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "input": {
                        "type": "string",
                        "description": "Input to the tool"
                    }
                },
                "required": ["input"]
            }),
            |args: HashMap<String, serde_json::Value>, _context| async move {
                let input = args.get("input")
                    .ok_or_else(|| AdkError::InvalidRequest("Missing input parameter".to_string()))?
                    .as_str()
                    .ok_or_else(|| AdkError::InvalidRequest("Input must be a string".to_string()))?;
                
                Ok(serde_json::json!({
                    "result": format!("Function processed: {}", input)
                }))
            }
        );
        
        // Create invocation context
        let session_service = Arc::new(InMemorySessionService::new());
        let session = session_service.create_session(
            "test-app",
            "test-user",
            None,
            Some("test-session"),
        ).await.unwrap();
        
        let context = InvocationContext {
            artifact_service: None,
            session_service: session_service.clone(),
            memory_service: None,
            invocation_id: "test-invocation".to_string(),
            branch: None,
            agent: Arc::new(TestAgent),
            user_content: None,
            session,
            end_invocation: false,
            run_config: RunConfig::default(),
            active_streaming_tools: HashMap::new(),
        };
        
        let tool_context = ToolContext {
            invocation_context: &context,
            execution_id: "test-execution".to_string(),
        };
        
        // Execute the tool
        let args = HashMap::from([
            ("input".to_string(), serde_json::json!("test input")),
        ]);
        
        let result = tool.execute(args, tool_context).await.unwrap();
        
        // Check result
        assert_eq!(result["result"], "Function processed: test input");
    }
    
    #[tokio::test]
    async fn test_tool_error_handling() {
        // Create a function tool with error handling
        let tool = function_tool!(
            "error-tool",
            "A tool that might error",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "should_error": {
                        "type": "boolean",
                        "description": "Whether the tool should error"
                    }
                },
                "required": ["should_error"]
            }),
            |args: HashMap<String, serde_json::Value>, _context| async move {
                let should_error = args.get("should_error")
                    .ok_or_else(|| AdkError::InvalidRequest("Missing should_error parameter".to_string()))?
                    .as_bool()
                    .ok_or_else(|| AdkError::InvalidRequest("should_error must be a boolean".to_string()))?;
                
                if should_error {
                    return Err(AdkError::ToolExecution("Tool error requested".to_string()));
                }
                
                Ok(serde_json::json!({
                    "result": "No error occurred"
                }))
            }
        );
        
        // Create invocation context
        let session_service = Arc::new(InMemorySessionService::new());
        let session = session_service.create_session(
            "test-app",
            "test-user",
            None,
            Some("test-session"),
        ).await.unwrap();
        
        let context = InvocationContext {
            artifact_service: None,
            session_service: session_service.clone(),
            memory_service: None,
            invocation_id: "test-invocation".to_string(),
            branch: None,
            agent: Arc::new(TestAgent),
            user_content: None,
            session,
            end_invocation: false,
            run_config: RunConfig::default(),
            active_streaming_tools: HashMap::new(),
        };
        
        let tool_context = ToolContext {
            invocation_context: &context,
            execution_id: "test-execution".to_string(),
        };
        
        // Test successful execution
        let args = HashMap::from([
            ("should_error".to_string(), serde_json::json!(false)),
        ]);
        
        let result = tool.execute(args, tool_context.clone()).await.unwrap();
        assert_eq!(result["result"], "No error occurred");
        
        // Test error case
        let args = HashMap::from([
            ("should_error".to_string(), serde_json::json!(true)),
        ]);
        
        let result = tool.execute(args, tool_context).await;
        assert!(result.is_err());
        match result {
            Err(AdkError::ToolExecution(msg)) => {
                assert_eq!(msg, "Tool error requested");
            }
            _ => panic!("Expected ToolExecution error"),
        }
    }
    
    // A simple test agent for testing context
    struct TestAgent;
    
    #[async_trait]
    impl crate::adk::agents::base_agent::BaseAgent for TestAgent {
        fn name(&self) -> &str {
            "test-agent"
        }
        
        fn description(&self) -> &str {
            "Test agent for unit tests"
        }
        
        async fn run_async<'a>(
            &'a self,
            _context: InvocationContext,
        ) -> Result<Pin<Box<dyn futures::Stream<Item = crate::adk::events::event::Event> + Send + 'a>>, AdkError> {
            // Not used in these tests
            unimplemented!()
        }
    }
}