// ===============================
// src/trigger_loom/loom.rs
// ===============================

use crate::invocation::invocation::Invocation;
use crate::invocation::named_construct::NamedConstruct;
use crate::trigger_loom::emotional_state::EmotionalState;
use crate::trigger_loom::glyph_matcher::{match_glyph, GlyphMatchResult};

pub fn thread_resonance(
    invocation: &Invocation,
    state: &EmotionalState,
) -> Option<GlyphMatchResult> {
    if invocation.resonance_required && state.is_resonant(0.5) {
        let phrase = &invocation.phrase;
        let known = vec![
            "Let structure echo symbol",
            "Validate this scroll",
            "Mark cycle",
        ];
        Some(match_glyph(phrase, &known))
    } else {
        None
    }
}

pub fn evaluate_construct(construct: &dyn NamedConstruct) -> f32 {
    // Placeholder: use construct's emotion/cadence/traits to calculate symbolic cost
    1.0
}
