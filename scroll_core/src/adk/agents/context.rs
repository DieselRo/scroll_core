// ==================================
// src/adk/agents/context.rs
// ==================================

use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::adk::agents::base_agent::BaseAgent;
use crate::adk::artifacts::base_artifact_service::BaseArtifactService;
use crate::adk::common::config::RunConfig;
use crate::adk::common::types::Content;
use crate::adk::memory::memory_service::BaseMemoryService;
use crate::adk::sessions::base_session_service::BaseSessionService;
use crate::adk::sessions::session::Session;

/// Context for agent invocation
#[derive(Clone)]
pub struct InvocationContext {
    /// Optional artifact service
    pub artifact_service: Option<Arc<dyn BaseArtifactService>>,

    /// Session service
    pub session_service: Arc<dyn BaseSessionService>,

    /// Optional memory service
    pub memory_service: Option<Arc<dyn BaseMemoryService>>,

    /// Unique invocation ID
    pub invocation_id: String,

    /// Optional branch ID for multi-branch conversations
    pub branch: Option<String>,

    /// The agent being invoked
    pub agent: Arc<dyn BaseAgent>,

    /// Optional user content that triggered this invocation
    pub user_content: Option<Content>,

    /// The session this invocation belongs to
    pub session: Session,

    /// Whether to end the invocation after this run
    pub end_invocation: bool,

    /// Run configuration
    pub run_config: RunConfig,

    /// Active streaming tools
    pub active_streaming_tools: HashMap<String, HashMap<String, String>>,
}

impl InvocationContext {
    /// Create a new invocation context
    pub fn new(
        artifact_service: Option<Arc<dyn BaseArtifactService>>,
        session_service: Arc<dyn BaseSessionService>,
        memory_service: Option<Arc<dyn BaseMemoryService>>,
        agent: Arc<dyn BaseAgent>,
        session: Session,
        user_content: Option<Content>,
        run_config: RunConfig,
    ) -> Self {
        Self {
            artifact_service,
            session_service,
            memory_service,
            invocation_id: new_invocation_context_id(),
            branch: None,
            agent,
            user_content,
            session,
            end_invocation: false,
            run_config,
            active_streaming_tools: HashMap::new(),
        }
    }
}

/// Create a new random invocation context ID
pub fn new_invocation_context_id() -> String {
    Uuid::new_v4().to_string()
}

/// Context for tool execution
#[derive(Clone)]
pub struct ToolContext<'a> {
    /// The invocation context this tool execution belongs to
    pub invocation_context: &'a InvocationContext,

    /// Tool execution ID
    pub execution_id: String,
}

impl<'a> ToolContext<'a> {
    /// Create a new tool context
    pub fn new(invocation_context: &'a InvocationContext) -> Self {
        Self {
            invocation_context,
            execution_id: Uuid::new_v4().to_string(),
        }
    }
}
