# Agent Development Kit (ADK) for Rust

This module provides a Rust implementation of the Google Agent Development Kit (ADK). It aims to provide similar functionality to the original Python ADK, but with an idiomatic Rust API and integration with the Scroll Core system.

## Features

- Agent framework for building and orchestrating AI agents
- Session management for maintaining conversations
- Tool system for giving agents capabilities
- Memory services for storing and retrieving history
- Artifact management for handling files and content

## Core Components

### Agents

The agent system is built around the `BaseAgent` trait, which defines the core capabilities of an agent:

```rust
#[async_trait]
pub trait BaseAgent: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    
    async fn run_async<'a>(
        &'a self,
        context: InvocationContext,
    ) -> Result<impl Stream<Item = Event>, AdkError>;
    
    // Default implementations
    fn find_agent(&self, name: &str) -> Option<&dyn BaseAgent>;
    fn find_sub_agent(&self, name: &str) -> Option<&dyn BaseAgent>;
}
```

### Runner

The `Runner` is the orchestration component that manages agents, sessions, and services:

```rust
// Create an agent
let agent = Arc::new(LlmAgent::new(
    "assistant".to_string(), 
    "A helpful assistant".to_string(),
    model,
    "You are a helpful assistant.".to_string(),
));

// Create a runner
let runner = new_in_memory_runner("my-app".to_string(), agent);

// Run the agent
let event_stream = runner.run(
    "user-123",
    "session-456",
    Some(user_message),
    RunConfig::default(),
).await?;
```

### Sessions

Sessions manage conversation state and events:

```rust
// Get a session
let session = session_service.get_session(
    "my-app",
    "user-123",
    "session-456",
    None,
).await?;

// Add an event
session_service.append_event(&mut session, event).await?;
```

### Tools

Tools provide capabilities to agents:

```rust
// Create a function tool
let weather_tool = function_tool!(
    "get_weather",
    "Get the weather for a location",
    serde_json::json!({
        "type": "object",
        "properties": {
            "location": {
                "type": "string",
                "description": "The location to get weather for"
            }
        },
        "required": ["location"]
    }),
    |args, _context| async move {
        // Tool implementation
        let location = args.get("location").unwrap().as_str().unwrap();
        Ok(serde_json::json!({ "temperature": 75, "condition": "sunny" }))
    }
);

// Add tool to agent
let agent = LlmAgent::new(/* ... */)
    .with_tools(vec![Arc::new(weather_tool)]);
```

## Examples

See `src/bin/adk_example.rs` for a simple example of how to use the ADK.

## Getting Started

```rust
// 1. Create an LLM
let model = Arc::new(MyLlm::new());

// 2. Create an agent
let agent = Arc::new(LlmAgent::new(
    "assistant".to_string(),
    "A helpful assistant".to_string(),
    model,
    "You are a helpful assistant.".to_string(),
));

// 3. Create a runner
let runner = new_in_memory_runner("my-app".to_string(), agent);

// 4. Run the agent
let event_stream = runner.run(
    "user-123",
    "session-456",
    Some(user_message),
    RunConfig::default(),
).await?;

// 5. Process events
while let Some(event) = event_stream.next().await {
    // Handle event
}
```

## Implementing a Custom LLM

```rust
struct MyLlm;

#[async_trait]
impl BaseLlm for MyLlm {
    fn model(&self) -> &str {
        "my-llm-1.0"
    }
    
    fn supported_models() -> Vec<String> {
        vec!["my-llm-1.0".to_string()]
    }
    
    async fn generate_content<'a>(
        &'a self,
        request: LlmRequest,
        stream: bool,
    ) -> Result<Pin<Box<dyn Stream<Item = LlmResponse> + Send + 'a>>, AdkError> {
        // Implementation
    }
}
```