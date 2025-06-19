// ==================================
// src/adk/models/llm_request.rs
// ==================================

use serde::{Deserialize, Serialize};

use crate::adk::common::types::Content;

/// Request to an LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmRequest {
    /// Model identifier
    pub model: String,

    /// Request configuration
    pub config: LlmRequestConfig,

    /// Message contents
    pub contents: Vec<Content>,
}

/// Configuration for LLM requests
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmRequestConfig {
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,

    /// Temperature (0.0 to 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Top-p (0.0 to 1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Top-k
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,

    /// Whether to return function calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_function_calls: Option<bool>,

    /// Response schema for structured output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_schema: Option<serde_json::Value>,

    /// Function declarations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_declarations: Option<Vec<serde_json::Value>>,
}

impl Default for LlmRequestConfig {
    fn default() -> Self {
        Self {
            max_tokens: None,
            temperature: Some(0.7),
            top_p: Some(0.95),
            top_k: None,
            return_function_calls: Some(true),
            response_schema: None,
            function_declarations: None,
        }
    }
}
