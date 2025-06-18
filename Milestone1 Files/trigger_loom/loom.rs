// ===============================
// src/trigger_loom/loom.rs
// ===============================

use crate::trigger_loom::emotional_state::EmotionalState;
use crate::trigger_loom::glyph_matcher::{match_glyph, GlyphMatchResult};
use crate::invocation::invocation::Invocation;

pub fn thread_resonance(invocation: &Invocation, state: &EmotionalState) -> Option<GlyphMatchResult> {
    if invocation.resonance_required && state.is_resonant(0.5) {
        let phrase = &invocation.phrase;
        let known = vec!["Let structure echo symbol", "Validate this scroll", "Mark cycle"];
        Some(match_glyph(phrase, &known))
    } else {
        None
    }
}
