// ==================================
// src/adk/agents/base_agent.rs
// ==================================

use async_trait::async_trait;
use futures::stream::Stream;
use futures::StreamExt;
use std::pin::Pin;
use std::sync::Arc;

use crate::adk::agents::context::InvocationContext;
use crate::adk::common::error::AdkError;
use crate::adk::events::event::Event;

/// Base trait for all agent implementations
#[async_trait]
pub trait BaseAgent: Send + Sync {
    /// Get the agent's name
    fn name(&self) -> &str;
    
    /// Get the agent's description
    fn description(&self) -> &str;
    
    /// Run the agent asynchronously
    async fn run_async<'a>(
        &'a self,
        context: InvocationContext,
    ) -> Result<Pin<Box<dyn Stream<Item = Event> + Send + 'a>>, AdkError>;
    
    /// Run the agent in live mode (e.g., for audio/video)
    async fn run_live<'a>(
        &'a self,
        context: InvocationContext,
    ) -> Result<Pin<Box<dyn Stream<Item = Event> + Send + 'a>>, AdkError> {
        // Default implementation just calls run_async
        self.run_async(context).await
    }
    
    /// Find an agent by name in the hierarchy
    fn find_agent(&self, name: &str) -> Option<Arc<dyn BaseAgent>> {
        if self.name() == name {
            // We can't create an Arc from self directly
            // This is a limitation of the design that should be addressed
            None
        } else {
            self.find_sub_agent(name)
        }
    }
    
    /// Find a sub-agent by name
    fn find_sub_agent(&self, _name: &str) -> Option<Arc<dyn BaseAgent>> {
        None
    }
}