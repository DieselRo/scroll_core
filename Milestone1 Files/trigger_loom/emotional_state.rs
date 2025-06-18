// ===============================
// src/trigger_loom/emotional_state.rs
// ===============================

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct EmotionalState {
    pub mood_trace: Vec<String>,
    pub intensity: f32,
    pub sigil_hint: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl EmotionalState {
    pub fn new(trace: Vec<String>, intensity: f32, sigil: Option<String>) -> Self {
        EmotionalState {
            mood_trace: trace,
            intensity,
            sigil_hint: sigil,
            timestamp: Utc::now(),
        }
    }

    pub fn is_resonant(&self, threshold: f32) -> bool {
        self.intensity >= threshold
    }
}
