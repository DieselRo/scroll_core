// ==================================
// src/adk/runner.rs
// ==================================

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use uuid::Uuid;

use crate::adk::agents::base_agent::BaseAgent;
use crate::adk::agents::context::{new_invocation_context_id, InvocationContext};
use crate::adk::artifacts::base_artifact_service::BaseArtifactService;
use crate::adk::common::config::RunConfig;
use crate::adk::common::error::AdkError;
use crate::adk::common::types::Content;
use crate::adk::events::event::Event;
use crate::adk::memory::memory_service::BaseMemoryService;
use crate::adk::sessions::base_session_service::BaseSessionService;
use crate::adk::sessions::session::Session;

/// Runner for agents
pub struct Runner {
    /// Application name
    pub app_name: String,
    
    /// Root agent
    pub agent: Arc<dyn BaseAgent>,
    
    /// Artifact service
    pub artifact_service: Option<Arc<dyn BaseArtifactService>>,
    
    /// Session service
    pub session_service: Arc<dyn BaseSessionService>,
    
    /// Memory service
    pub memory_service: Option<Arc<dyn BaseMemoryService>>,
}

impl Runner {
    /// Create a new runner
    pub fn new(
        app_name: String,
        agent: Arc<dyn BaseAgent>,
        session_service: Arc<dyn BaseSessionService>,
    ) -> Self {
        Self {
            app_name,
            agent,
            artifact_service: None,
            session_service,
            memory_service: None,
        }
    }
    
    /// Set the artifact service
    pub fn with_artifact_service(mut self, artifact_service: Arc<dyn BaseArtifactService>) -> Self {
        self.artifact_service = Some(artifact_service);
        self
    }
    
    /// Set the memory service
    pub fn with_memory_service(mut self, memory_service: Arc<dyn BaseMemoryService>) -> Self {
        self.memory_service = Some(memory_service);
        self
    }
    
    /// Run an agent with the given input
    pub async fn run<'a>(
        &'a self,
        user_id: &str,
        session_id: &str,
        new_message: Option<Content>,
        run_config: RunConfig,
    ) -> Result<Pin<Box<dyn Stream<Item = Event> + Send + 'a>>, AdkError> {
        // Get or create the session
        let session = match self.session_service.get_session(
            &self.app_name,
            user_id,
            session_id,
            None,
        ).await? {
            Some(session) => session,
            None => {
                self.session_service.create_session(
                    &self.app_name,
                    user_id,
                    None,
                    Some(session_id),
                ).await?
            }
        };
        
        // Create invocation context
        let mut invocation_context = self.new_invocation_context(
            session,
            new_message,
            run_config,
        );
        
        // Find the agent to run
        invocation_context.agent = self.find_agent_to_run(&invocation_context.session);
        
        // Append new message to session if provided
        if let Some(content) = &invocation_context.user_content {
            let event = Event::new_user_event(&invocation_context.invocation_id, content.clone());
            let mut session = invocation_context.session.clone();
            self.session_service.append_event(&mut session, event).await?;
            invocation_context.session = session;
        }
        
        // Run the agent - create a clone of the context to avoid ownership issues
        let context_clone = invocation_context.clone();
        let event_stream = self
            .agent
            .run_async(context_clone)
            .await?
            .boxed();
        
        // Create a stream that appends non-partial events to the session
        let session_service = self.session_service.clone();
        let app_name = self.app_name.clone();
        let user_id = user_id.to_string();
        let session_id = session_id.to_string();
        
        let stream = event_stream
            .filter_map(move |event| {
                let session_service = session_service.clone();
                let app_name = app_name.clone();
                let user_id = user_id.clone();
                let session_id = session_id.clone();
            
            async move {
                // Save non-partial events to the session
                if !event.partial.unwrap_or(false) {
                    if let Ok(Some(mut session)) = session_service.get_session(
                        &app_name,
                        &user_id,
                        &session_id,
                        None,
                    ).await {
                        // Ignore errors - we still want to return the event
                        let _ = session_service.append_event(&mut session, event.clone()).await;
                    }
                }
                
                Some(event)
            }
        })
        .boxed();

        Ok(stream)
    }
    
    /// Create a new invocation context
    fn new_invocation_context(
        &self,
        session: Session,
        new_message: Option<Content>,
        run_config: RunConfig,
    ) -> InvocationContext {
        InvocationContext {
            artifact_service: self.artifact_service.clone(),
            session_service: self.session_service.clone(),
            memory_service: self.memory_service.clone(),
            invocation_id: new_invocation_context_id(),
            branch: None,
            agent: self.agent.clone(),
            user_content: new_message,
            session,
            end_invocation: false,
            run_config,
            active_streaming_tools: HashMap::new(),
        }
    }
    
    /// Find the agent to run based on the session
    fn find_agent_to_run(&self, session: &Session) -> Arc<dyn BaseAgent> {
        // Start with the root agent
        let root_agent = self.agent.clone();
        
        // If no events in the session, just use the root agent
        if session.events.is_empty() {
            return root_agent;
        }
        
        // Find the last agent that sent a message
        for event in session.events.iter().rev() {
            // Skip user messages
            if event.author == "user" {
                continue;
            }
            
            // If the last agent was the root agent, use that
            if event.author == root_agent.name() {
                return root_agent;
            }
            
            // Otherwise, try to find the agent by name
            if event.author == root_agent.name() {
                return root_agent;
            } else if let Some(agent) = root_agent.find_sub_agent(&event.author) {
                return agent;
            }
        }
        
        // Default to the root agent
        root_agent
    }
    
    /// Close a session
    pub async fn close_session(&self, session: &mut Session) -> Result<(), AdkError> {
        // Add session to memory if memory service is available
        if let Some(memory_service) = &self.memory_service {
            memory_service.add_session_to_memory(session.clone()).await?;
        }
        
        // Close the session
        self.session_service.close_session(session).await
    }
}

/// In-memory runner implementation for testing and development
pub mod in_memory {
    use super::*;
    use crate::adk::sessions::in_memory_service::InMemorySessionService;
    
    /// Create a new in-memory runner
    pub fn new_in_memory_runner(
        app_name: String,
        agent: Arc<dyn BaseAgent>,
    ) -> Runner {
        let session_service = Arc::new(InMemorySessionService::new());
        
        Runner {
            app_name,
            agent,
            artifact_service: None,
            session_service,
            memory_service: None,
        }
    }
}