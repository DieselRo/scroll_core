// src/models/base_model.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponseContent {
    pub text: String,
}
