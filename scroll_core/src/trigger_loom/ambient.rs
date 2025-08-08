use crate::trigger_loom::emotional_state::EmotionalState;

/// Returns true if ambient trigger should fire based on tags and emotion intensity threshold.
pub fn should_trigger(tags: &[String], mood: &EmotionalState, watch_tag: &str, intensity_threshold: f32) -> bool {
    if !mood.is_resonant(intensity_threshold) {
        return false;
    }
    tags.iter().any(|t| t.eq_ignore_ascii_case(watch_tag))
}


