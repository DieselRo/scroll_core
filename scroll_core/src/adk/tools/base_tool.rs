// ==================================
// src/adk/tools/base_tool.rs
// ==================================

use async_trait::async_trait;
use std::collections::HashMap;

use crate::adk::agents::context::ToolContext;
use crate::adk::common::error::AdkError;
use crate::adk::common::types::FunctionDeclaration;

/// Base trait for tool implementations
#[async_trait]
pub trait BaseTool: Send + Sync {
    /// Get the tool's name
    fn name(&self) -> &str;

    /// Get the tool's description
    fn description(&self) -> &str;

    /// Get the tool's function declaration
    fn declaration(&self) -> FunctionDeclaration;

    /// Execute the tool with provided arguments
    async fn execute(
        &self,
        args: HashMap<String, serde_json::Value>,
        context: ToolContext<'_>,
    ) -> Result<serde_json::Value, AdkError>;
}
