// ==================================
// src/adk/memory/memory_service.rs
// ==================================

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::adk::common::error::AdkError;
use crate::adk::sessions::session::Session;

/// Base trait for memory service implementations
#[async_trait]
pub trait BaseMemoryService: Send + Sync {
    /// Add a session to memory
    async fn add_session_to_memory(&self, session: Session) -> Result<(), AdkError>;
    
    /// Retrieve memory for a user in an app
    async fn retrieve_memory(
        &self,
        app_name: &str,
        user_id: &str,
        query: &str,
    ) -> Result<Vec<MemoryEntry>, AdkError>;
    
    /// Retrieve session from memory
    async fn retrieve_session(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
    ) -> Result<Option<Session>, AdkError>;
}

/// Memory entry returned by memory service
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryEntry {
    /// Unique identifier for this memory entry
    pub id: String,
    
    /// The session this memory entry belongs to
    pub session_id: String,
    
    /// The content of this memory entry
    pub content: String,
    
    /// Relevance score (0.0 to 1.0)
    pub relevance: f64,
}