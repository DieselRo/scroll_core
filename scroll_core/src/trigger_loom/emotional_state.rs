// ===============================
// src/trigger_loom/emotional_state.rs
// ===============================

use crate::chat::chat_session::ChatMessage;
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

    /// Update the emotional intensity based on a chat message.
    /// Currently increases intensity by 0.1 when the message contains `":)"`.
    pub fn update_from_message(&mut self, message: &ChatMessage) {
        if message.content.contains(":)") {
            self.intensity = (self.intensity + 0.1).min(1.0);
        }
    }
}
