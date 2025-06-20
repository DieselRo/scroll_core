use chrono::{Duration, Utc};
use scroll_core::core::cost_manager::{ContextScorer, SemanticContextScorer};
use scroll_core::invocation::invocation::{Invocation, InvocationMode, InvocationTier};
use scroll_core::Scroll;
use uuid::Uuid;
fn make_invocation() -> Invocation {
    Invocation {
        id: Uuid::new_v4(),
        phrase: "invoke".into(),
        invoker: "tester".into(),
        invoked: "construct".into(),
        tier: InvocationTier::True,
        mode: InvocationMode::Read,
        resonance_required: false,
        timestamp: Utc::now(),
    }
}

#[test]
fn test_semantic_score_influences() {
    let scorer = SemanticContextScorer;
    let inv = make_invocation();
    let now = Utc::now();
    let scroll = Scroll::builder("Test")
        .body("Body")
        .invocation_phrase("Invoke")
        .sigil("🔮")
        .last_modified(now)
        .build();
    let high = scorer.score(&inv, &[scroll.clone()], 0.9);
    let low = scorer.score(&inv, &[scroll], 0.1);
    assert!(high > low);
}

#[test]
fn test_semantic_closer_ranked_higher() {
    let scorer = SemanticContextScorer;
    let inv = make_invocation();
    let scroll_recent = Scroll::builder("Test")
        .body("Body")
        .invocation_phrase("Invoke")
        .sigil("🔮")
        .last_modified(Utc::now())
        .build();
    let scroll_old = Scroll::builder("Test")
        .body("Body")
        .invocation_phrase("Invoke")
        .sigil("🔮")
        .last_modified(Utc::now() - Duration::days(10))
        .build();
    let score_recent = scorer.score(&inv, &[scroll_recent], 0.8);
    let score_old = scorer.score(&inv, &[scroll_old], 0.2);
    assert!(score_recent > score_old);
}
