// ==================================
// src/adk/models/registry.rs
// ==================================

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::adk::common::error::AdkError;
use crate::adk::models::base_llm::BaseLlm;

/// Registry for LLM models
#[derive(Default)]
pub struct ModelRegistry {
    /// Registered models (model name -> implementation)
    models: Mutex<HashMap<String, Arc<dyn BaseLlm>>>,
}

impl ModelRegistry {
    /// Create a new model registry
    pub fn new() -> Self {
        Self {
            models: Mutex::new(HashMap::new()),
        }
    }
    
    /// Register a model
    pub fn register<T: BaseLlm + 'static>(&self, model: T) -> Result<(), AdkError> {
        let model_name = model.model().to_string();
        let model = Arc::new(model);
        
        let mut models = self.models.lock().map_err(|e| AdkError::Other(e.to_string()))?;
        models.insert(model_name, model);
        
        Ok(())
    }
    
    /// Get a model by name
    pub fn get(&self, model_name: &str) -> Result<Arc<dyn BaseLlm>, AdkError> {
        let models = self.models.lock().map_err(|e| AdkError::Other(e.to_string()))?;
        
        models
            .get(model_name)
            .cloned()
            .ok_or_else(|| AdkError::NotFound(format!("Model not found: {}", model_name)))
    }
    
    /// List all registered models
    pub fn list(&self) -> Result<Vec<String>, AdkError> {
        let models = self.models.lock().map_err(|e| AdkError::Other(e.to_string()))?;
        
        Ok(models.keys().cloned().collect())
    }
}