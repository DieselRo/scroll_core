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
    // New fields for trigger patterns and sentiment
    pub trigger_patterns: Vec<String>,
    pub sentiment: f32,
    /// Deterministic seed used for demos/tests.
    pub seed: u64,
    /// Default decay rate applied each tick.
    pub decay_rate: f32,
}

impl EmotionalState {
    pub fn new(trace: Vec<String>, intensity: f32, sigil: Option<String>) -> Self {
        EmotionalState {
            mood_trace: trace,
            intensity,
            sigil_hint: sigil,
            timestamp: Utc::now(),
            trigger_patterns: Vec::new(),
            sentiment: 0.0,
            seed: 0,
            decay_rate: 0.01,
        }
    }

    pub fn is_resonant(&self, threshold: f32) -> bool {
        self.intensity >= threshold
    }

    /// Update the emotional intensity based on a chat message.
    /// Currently increases intensity by 0.1 when the message contains ":)".
    pub fn update_from_message(&mut self, message: &ChatMessage) {
        if message.content.contains(":)") {
            self.intensity = (self.intensity + 0.1).min(1.0);
        }
        if message.content.contains('!') {
            self.sentiment = (self.sentiment + 0.05).min(1.0);
        }
        self.timestamp = Utc::now();
    }

    /// Recency decay (e.g., 0.01 per second)
    pub fn decay(&mut self, per_sec: f32) {
        let now = Utc::now();
        let elapsed = (now - self.timestamp).num_seconds().max(0) as f32;
        let decay = (elapsed * per_sec).min(1.0);
        self.intensity = (self.intensity - decay).max(0.0);
        self.sentiment = (self.sentiment - decay).max(0.0);
        self.timestamp = now;
    }

    /// Decay using the state's stored decay rate.
    pub fn decay_step(&mut self) {
        let rate = self.decay_rate;
        self.decay(rate);
    }
}
