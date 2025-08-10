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

/// Profiles provide handy presets for deterministic CI runs or richer demos.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerLoopProfile {
    Ci,
    Demo,
}

impl TriggerLoopProfile {
    /// Return a base configuration tuned for the profile. Callers may override
    /// the fields afterwards (e.g. via CLI flags).
    pub fn config(self) -> TriggerLoopConfig {
        match self {
            TriggerLoopProfile::Ci => TriggerLoopConfig {
                rhythm: SymbolicRhythm::Constant(1.0),
                max_invocations_per_tick: 1,
                allow_test_ticks: true,
                emotional_signature: None,
            },
            TriggerLoopProfile::Demo => TriggerLoopConfig {
                rhythm: SymbolicRhythm::Constant(0.5),
                max_invocations_per_tick: 2,
                allow_test_ticks: true,
                emotional_signature: None,
            },
        }
    }

    /// Optional tick cap used by CI profile to prevent infinite loops.
    pub fn tick_limit(self) -> Option<u64> {
        match self {
            TriggerLoopProfile::Ci => Some(3),
            TriggerLoopProfile::Demo => None,
        }
    }
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
