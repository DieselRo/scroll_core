// ===============================
// src/trigger_loom/config.rs
// ===============================

use crate::trigger_loom::emotion::modulate_frequency;
use crate::EmotionSignature;
use chrono::{Local, TimeZone, Timelike};

#[derive(Debug, Clone)]
pub enum SymbolicRhythm {
    Constant(f32), // Hz
    Dawn,
    Dusk,
    Spiral(u32), // Recursive step rhythm
    EmotionDriven,
}

#[derive(Debug, Clone)]
pub struct TriggerLoopConfig {
    pub rhythm: SymbolicRhythm,
    pub max_invocations_per_tick: usize,
    pub allow_test_ticks: bool,
    pub emotional_signature: Option<EmotionSignature>,
}

impl TriggerLoopConfig {
    pub fn resolve_frequency(&self) -> f32 {
        self.resolve_frequency_at(Local::now())
    }

    pub fn resolve_frequency_at<Tz: TimeZone>(&self, now: chrono::DateTime<Tz>) -> f32 {
        match &self.rhythm {
            SymbolicRhythm::Constant(hz) => *hz,
            SymbolicRhythm::EmotionDriven => {
                if let Some(emotion) = &self.emotional_signature {
                    modulate_frequency(1.0, emotion)
                } else {
                    1.0
                }
            }
            SymbolicRhythm::Dawn => {
                let hour = now.with_timezone(&Local).hour();
                if !(6..=21).contains(&hour) {
                    0.0
                } else {
                    1.0
                }
            }
            SymbolicRhythm::Dusk => {
                let hour = now.with_timezone(&Local).hour();
                if (6..18).contains(&hour) {
                    0.0
                } else {
                    1.0
                }
            }
            SymbolicRhythm::Spiral(n) => {
                let step = *n as f32;
                if step == 0.0 {
                    1.0
                } else {
                    1.0 / step
                }
            }
        }
    }
}
