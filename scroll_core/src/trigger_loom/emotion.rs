// ===============================
// src/trigger_loom/emotion.rs
// ===============================

use crate::schema::EmotionSignature;

/// Map emotional intensity to a frequency in Hz.
/// Intensity 0.0 corresponds to `base * 0.1` Hz and
/// intensity 1.0 corresponds to `base * 2.0` Hz.
/// If the tone is `"urgent"`, the resulting frequency is amplified by 1.5.
pub fn modulate_frequency(base: f32, emotion: &EmotionSignature) -> f32 {
    let intensity = emotion.intensity.unwrap_or(0.0).clamp(0.0, 1.0);
    let mut freq = base * (0.1 + intensity * 1.9);
    if emotion.tone.eq_ignore_ascii_case("urgent") {
        freq *= 1.5;
    }
    freq
}
