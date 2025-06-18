// ==================================
// src/adk/agents/llm_agent.rs
// ==================================

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use futures::stream;
use std::pin::Pin;
use std::sync::Arc;
use std::collections::HashMap;

use crate::adk::agents::base_agent::BaseAgent;
use crate::adk::agents::context::InvocationContext;
use crate::adk::common::config::IncludeContents;
use crate::adk::common::error::AdkError;
use crate::adk::common::types::{Content, Part, FunctionCall};
use crate::adk::events::event::Event;
use crate::adk::events::event_actions::{EventActions, TransferToAgentAction};
use crate::adk::models::base_llm::BaseLlm;
use crate::adk::models::llm_request::{LlmRequest, LlmRequestConfig};
use crate::adk::tools::base_tool::BaseTool;

/// LLM-based agent implementation
#[derive(Clone)]
pub struct LlmAgent {
    /// Agent name
    pub name: String,
    
    /// Agent description
    pub description: String,
    
    /// LLM model to use
    pub model: Arc<dyn BaseLlm>,
    
    /// Agent instruction/system prompt
    pub instruction: String,
    
    /// Global instruction to include with every message
    pub global_instruction: String,
    
    /// Tools available to this agent
    pub tools: Vec<Arc<dyn BaseTool>>,
    
    /// Sub-agents that this agent can transfer to
    pub sub_agents: Vec<Arc<dyn BaseAgent>>,
    
    /// Whether to disallow transfer to parent agent
    pub disallow_transfer_to_parent: bool,
    
    /// Whether to disallow transfer to peer agents
    pub disallow_transfer_to_peers: bool,
    
    /// What contents to include in the context
    pub include_contents: IncludeContents,
    
    /// Parent agent (if any)
    pub parent_agent: Option<Arc<dyn BaseAgent>>,
}

impl LlmAgent {
    /// Create a new LLM agent
    pub fn new(
        name: String,
        description: String,
        model: Arc<dyn BaseLlm>,
        instruction: String,
    ) -> Self {
        Self {
            name,
            description,
            model,
            instruction,
            global_instruction: String::new(),
            tools: Vec::new(),
            sub_agents: Vec::new(),
            disallow_transfer_to_parent: false,
            disallow_transfer_to_peers: false,
            include_contents: IncludeContents::All,
            parent_agent: None,
        }
    }
    
    /// Builder method to add tools
    pub fn with_tools(mut self, tools: Vec<Arc<dyn BaseTool>>) -> Self {
        self.tools = tools;
        self
    }
    
    /// Builder method to add sub-agents
    pub fn with_sub_agents(mut self, sub_agents: Vec<Arc<dyn BaseAgent>>) -> Self {
        self.sub_agents = sub_agents;
        self
    }
    
    /// Builder method to set global instruction
    pub fn with_global_instruction(mut self, global_instruction: String) -> Self {
        self.global_instruction = global_instruction;
        self
    }
    
    /// Builder method to set parent agent
    pub fn with_parent_agent(mut self, parent_agent: Arc<dyn BaseAgent>) -> Self {
        self.parent_agent = Some(parent_agent);
        self
    }
    
    /// Builder method to set transfer options
    pub fn with_transfer_options(
        mut self,
        disallow_transfer_to_parent: bool,
        disallow_transfer_to_peers: bool,
    ) -> Self {
        self.disallow_transfer_to_parent = disallow_transfer_to_parent;
        self.disallow_transfer_to_peers = disallow_transfer_to_peers;
        self
    }
    
    /// Builder method to set include contents option
    pub fn with_include_contents(mut self, include_contents: IncludeContents) -> Self {
        self.include_contents = include_contents;
        self
    }
    
    /// Prepare function declarations for the LLM
    fn prepare_function_declarations(&self) -> Vec<serde_json::Value> {
        self.tools
            .iter()
            .map(|tool| {
                let decl = tool.declaration();
                serde_json::json!({
                    "name": decl.name,
                    "description": decl.description,
                    "parameters": decl.parameters
                })
            })
            .collect()
    }
    
    /// Build the conversation history for the LLM
    async fn build_conversation_history(
        &self,
        context: &InvocationContext,
    ) -> Result<Vec<Content>, AdkError> {
        let mut contents = Vec::new();
        
        // Add system message with instruction
        contents.push(Content {
            role: Some("system".to_string()),
            parts: vec![Part {
                text: Some(self.instruction.clone()),
                inline_data: None,
                function_call: None,
                function_response: None,
            }],
        });
        
        // Add conversation history from session events
        // This is a simplified implementation - in a real system,
        // you'd want to handle things like pagination, filtering, etc.
        for event in &context.session.events {
            if let Some(content) = &event.content {
                contents.push(content.clone());
            }
        }
        
        // Add user's new message if available
        if let Some(user_content) = &context.user_content {
            contents.push(user_content.clone());
        }
        
        Ok(contents)
    }
    
    /// Execute a tool
    async fn execute_tool(
        &self,
        function_call: FunctionCall,
        context: &InvocationContext,
    ) -> Result<serde_json::Value, AdkError> {
        let tool_name = &function_call.name;
        let args = function_call.args;
        
        // Find the tool
        let tool = self.tools
            .iter()
            .find(|t| t.name() == tool_name)
            .ok_or_else(|| AdkError::NotFound(format!("Tool not found: {}", tool_name)))?;
        
        // Create tool context and execute
        let tool_context = crate::adk::agents::context::ToolContext::new(context);
        tool.execute(args, tool_context).await
    }
    
    /// Handle agent transfer if requested
    fn handle_agent_transfer(
        &self,
        event: &mut Event,
    ) -> Option<String> {
        // Check for transfer_to_agent action
        if let Some(transfer) = &event.actions.transfer_to_agent {
            let target_agent = transfer.agent_name.clone();
            
            // Logic for agent transfer validation would go here
            // For now, just return the target agent name
            return Some(target_agent);
        }
        
        None
    }
}

#[async_trait]
impl BaseAgent for LlmAgent {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    async fn run_async<'a>(
        &'a self,
        context: InvocationContext,
    ) -> Result<Pin<Box<dyn Stream<Item = Event> + Send + 'a>>, AdkError> {
        // Build conversation history
        let contents = self.build_conversation_history(&context).await?;
        
        // Prepare LLM request
        let function_declarations = self.prepare_function_declarations();
        let request = LlmRequest {
            model: self.model.model().to_string(),
            config: LlmRequestConfig {
                function_declarations: Some(function_declarations),
                ..Default::default()
            },
            contents,
        };
        
        // Stream flag based on run config
        let stream = match context.run_config.streaming_mode {
            Some(crate::adk::common::config::StreamingMode::BlockWaitNone) => false,
            Some(crate::adk::common::config::StreamingMode::BlockWaitAll) => false,
            _ => true,
        };
        
        // Call the LLM
        let response_stream = self.model.generate_content(request, stream).await?;
        
        // Process the response stream
        let context_clone = context;
        let agent_name = self.name.clone();
        
        let stream = response_stream.map(move |response| {
            let partial = response.partial.unwrap_or(false);
            
            // Create event from response
            let mut event = Event::new_agent_event(
                &context_clone.invocation_id,
                &agent_name,
                response.content.unwrap_or_else(|| Content {
                    role: Some("assistant".to_string()),
                    parts: Vec::new(),
                }),
                partial,
            );
            
            // If this is an error response, set error fields
            if let Some(error_code) = response.error_code {
                event.error_code = Some(error_code);
                event.error_message = response.error_message;
            }
            
            event
        });
        
        Ok(Box::pin(stream))
    }
    
    fn find_sub_agent(&self, name: &str) -> Option<Arc<dyn BaseAgent>> {
        // First check direct sub-agents
        for agent in &self.sub_agents {
            if agent.name() == name {
                return Some(agent.clone());
            }
            
            // Then check recursive sub-agents
            if let Some(sub_agent) = agent.find_sub_agent(name) {
                return Some(sub_agent);
            }
        }
        
        None
    }
}