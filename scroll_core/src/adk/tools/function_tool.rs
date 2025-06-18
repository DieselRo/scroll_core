// ==================================
// src/adk/tools/function_tool.rs
// ==================================

use async_trait::async_trait;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::adk::agents::context::ToolContext;
use crate::adk::common::error::AdkError;
use crate::adk::common::types::FunctionDeclaration;
use crate::adk::tools::base_tool::BaseTool;

/// Type alias for async function tool callback
pub type AsyncToolFn = Arc<dyn Fn(HashMap<String, serde_json::Value>, ToolContext<'_>) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, AdkError>> + Send>> + Send + Sync>;

/// Function-based tool implementation
pub struct FunctionTool {
    /// Tool name
    name: String,
    
    /// Tool description
    description: String,
    
    /// Function declaration (parameters schema)
    declaration: FunctionDeclaration,
    
    /// Async function to execute
    function: AsyncToolFn,
}

impl FunctionTool {
    /// Create a new function tool
    pub fn new(
        name: String,
        description: String,
        parameters: serde_json::Value,
        function: AsyncToolFn,
    ) -> Self {
        let name_clone = name.clone();
        let description_clone = description.clone();
        Self {
            name: name_clone,
            description,
            declaration: FunctionDeclaration {
                name,
                description: description_clone,
                parameters,
            },
            function,
        }
    }
}

#[async_trait]
impl BaseTool for FunctionTool {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    fn declaration(&self) -> FunctionDeclaration {
        self.declaration.clone()
    }
    
    async fn execute(
        &self,
        args: HashMap<String, serde_json::Value>,
        context: ToolContext<'_>,
    ) -> Result<serde_json::Value, AdkError> {
        (self.function)(args, context).await
    }
}

/// Create a function tool with type checking
#[macro_export]
macro_rules! function_tool {
    ($name:expr, $description:expr, $parameters:expr, $function:expr) => {
        crate::adk::tools::function_tool::FunctionTool::new(
            $name.to_string(),
            $description.to_string(),
            $parameters,
            std::sync::Arc::new(move |args, context| {
                Box::pin(async move {
                    let result = $function(args, context).await?;
                    Ok(result)
                })
            }),
        )
    };
}