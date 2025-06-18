// mythic_heat.rs – Evaluator of Scroll Significance
//========================================================
#![allow(dead_code)]
#![allow(unused_imports)]


use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::schema::EmotionSignature;
use crate::archive::scroll_access_log::ScrollAccess;

/// Represents how "hot" or relevant a scroll is in memory.
#[derive(Debug, Clone)]
pub struct MythicHeat {
    pub scroll_id: Uuid,
    pub emotional_intensity: f32,  // 0.0 to 1.0
    pub access_count: usize,
    pub last_accessed: DateTime<Utc>,
    pub cost_weight: f32,          // symbolic/technical weight
}

impl MythicHeat {
    pub fn compute(
        scroll_id: Uuid,
        emotion: &EmotionSignature,
        access: &ScrollAccess,
        cost_weight: f32,
    ) -> Self {
        Self {
            scroll_id,
            emotional_intensity: emotion.intensity.unwrap_or(0.0),
            access_count: access.access_count,
            last_accessed: access.last_accessed,
            cost_weight,
        }
    }

    /// Returns a composite score from all weighted fields.
    pub fn score(&self) -> f32 {
        let emotion_factor = self.emotional_intensity * 2.0;
        let access_factor = (self.access_count as f32).sqrt();
        let recency_factor = {
            let elapsed = Utc::now().signed_duration_since(self.last_accessed);
            1.0 / ((elapsed.num_seconds().max(1)) as f32).log2() // decay with time
        };
        let cost_penalty = self.cost_weight;

        let raw_score = (emotion_factor + access_factor + recency_factor) - cost_penalty;
        raw_score.min(25.0) // Clamp max score
    }

    /// Provides a symbolic interpretation of the current score.
    pub fn symbolic_echo(&self) -> String {
        match self.score() {
            s if s > 15.0 => "🔥 A pillar of memory".into(),
            s if s > 10.0 => "🌟 Recently stirred".into(),
            s if s > 5.0 => "🌀 Still warm with echoes".into(),
            _ => "🌫️ Drifting toward silence".into(),
        }
    }

    /// Returns each weighted component for transparency and tuning.
    pub fn breakdown(&self) -> (f32, f32, f32, f32) {
        let emotion = self.emotional_intensity * 2.0;
        let access = (self.access_count as f32).sqrt();
        let recency = {
            let elapsed = Utc::now().signed_duration_since(self.last_accessed);
            1.0 / ((elapsed.num_seconds().max(1)) as f32).log2()
        };
        let cost = self.cost_weight;
        (emotion, access, recency, cost)
    }
}
