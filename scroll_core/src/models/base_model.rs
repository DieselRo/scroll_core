// src/models/base_model.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponseContent {
    pub text: String,
}
