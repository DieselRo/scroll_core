
use scroll_core::trigger_loom::emotional_state::EmotionalState;
use scroll_core::trigger_loom::glyph_matcher::{match_glyph, GlyphMatchResult};
use scroll_core::trigger_loom::loom::thread_resonance;
use scroll_core::invocation::invocation::{Invocation, InvocationTier, InvocationMode};
use uuid::Uuid;
use chrono::Utc;

#[test]
fn test_glyph_match_exact() {
    let signal = "Let structure echo symbol";
    let known = vec!["Let structure echo symbol", "Invoke wisdom"];
    assert!(matches!(match_glyph(signal, &known), GlyphMatchResult::Exact));
}

#[test]
fn test_thread_resonance_activation() {
    let invocation = Invocation {
        id: Uuid::new_v4(),
        phrase: "Let structure echo symbol".into(),
        invoker: "Tester".into(),
        invoked: "Validator".into(),
        tier: InvocationTier::Calling,
        mode: InvocationMode::Custom("Test".into()),
        resonance_required: true,
        timestamp: Utc::now(),
    };

    let emotion = EmotionalState::new(vec!["focused".into()], 0.7, None);
    let result = thread_resonance(&invocation, &emotion);
    assert!(matches!(result, Some(GlyphMatchResult::Exact)));
}
