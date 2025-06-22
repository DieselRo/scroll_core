//! Types for describing changes to construct memory between invocations.
//! Currently only exposes MemoryDelta used in tests.
//! See [Virelya](../../AGENTS.md#virelya) for future plans around memory resonance.
// src/memory/memory_result.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDelta {
    pub state_delta: HashMap<String, String>,
}
