// src/memory/memory_result.rs
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDelta {
    pub state_delta: HashMap<String, String>,
}