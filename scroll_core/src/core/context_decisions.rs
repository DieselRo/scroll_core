use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextDecision {
    pub construct: String,
    pub frame_id: Uuid,
    pub candidate_path: Option<String>,
    pub included: bool,
    pub reason: String,
    pub score: f32,
    pub recency_hours: f32,
    pub running_tokens: usize,
    pub max_tokens: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextFrameSummary {
    pub construct: String,
    pub frame_id: Uuid,
    pub max_tokens: usize,
    pub max_items: usize,
    pub min_relevance: f32,
    pub half_life_hours: f32,
    pub total_candidates: usize,
    pub included_count: usize,
    pub excluded_count: usize,
    pub build_ms: u128,
}

#[derive(Debug, Clone)]
pub struct ContextBuildReport {
    pub summary: ContextFrameSummary,
    pub decisions: Vec<ContextDecision>,
}
