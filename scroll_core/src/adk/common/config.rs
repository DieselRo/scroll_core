// ==================================
// src/adk/common/config.rs
// ==================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Run configuration for agent execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_mode: Option<StreamingMode>,

    #[serde(default)]
    pub support_cfc: bool,

    #[serde(default)]
    pub save_input_blobs_as_artifacts: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_modalities: Option<Vec<String>>,

    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub model_overrides: HashMap<String, String>,
}

impl Default for RunConfig {
    fn default() -> Self {
        Self {
            streaming_mode: Some(StreamingMode::Auto),
            support_cfc: false,
            save_input_blobs_as_artifacts: true,
            response_modalities: None,
            model_overrides: HashMap::new(),
        }
    }
}

/// Streaming mode for agent responses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamingMode {
    Auto,
    Streaming,
    BlockWaitNone,
    BlockWaitAll,
}

/// Content inclusion options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IncludeContents {
    All,
    Direct,
    None,
}
