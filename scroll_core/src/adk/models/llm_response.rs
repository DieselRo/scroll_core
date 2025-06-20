// ==================================
// src/adk/models/llm_response.rs
// ==================================

use serde::{Deserialize, Serialize};

use crate::adk::common::types::Content;

/// Response from an LLM
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LlmResponse {
    /// Response content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,

    /// Whether this is a partial response (streaming)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial: Option<bool>,

    /// Error code if an error occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,

    /// Error message if an error occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
