// ==================================
// src/adk/artifacts/base_artifact_service.rs
// ==================================

use async_trait::async_trait;

use crate::adk::common::error::AdkError;
use crate::adk::common::types::{Content, Part};

/// Base trait for artifact service implementations
#[async_trait]
pub trait BaseArtifactService: Send + Sync {
    /// Save an artifact
    async fn save_artifact(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
        filename: &str,
        artifact: Part,
    ) -> Result<String, AdkError>;
    
    /// Get an artifact
    async fn get_artifact(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
        filename: &str,
    ) -> Result<Part, AdkError>;
    
    /// Delete an artifact
    async fn delete_artifact(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
        filename: &str,
    ) -> Result<(), AdkError>;
    
    /// List artifacts for a session
    async fn list_artifacts(
        &self,
        app_name: &str,
        user_id: &str,
        session_id: &str,
    ) -> Result<Vec<String>, AdkError>;
}