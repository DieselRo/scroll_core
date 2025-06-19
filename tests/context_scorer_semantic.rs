use chrono::{Duration, Utc};
use scroll_core::core::cost_manager::{ContextScorer, SemanticContextScorer};
use scroll_core::invocation::invocation::{Invocation, InvocationMode, InvocationTier};
use scroll_core::{EmotionSignature, Scroll, ScrollOrigin, ScrollStatus, ScrollType, YamlMetadata};
use uuid::Uuid;

fn make_scroll(days_ago: i64) -> Scroll {
    let now = Utc::now() - Duration::days(days_ago);
    Scroll {
        id: Uuid::new_v4(),
        title: "Test".into(),
        scroll_type: ScrollType::Canon,
        yaml_metadata: YamlMetadata {
            title: "Test".into(),
            scroll_type: ScrollType::Canon,
            emotion_signature: EmotionSignature { tone: "neutral".into(), emphasis: 0.5, resonance: "flat".into(), intensity: Some(0.5) },
            tags: vec![],
            last_modified: Some(now),
            file_path: None,
        },
        markdown_body: "Body".into(),
        invocation_phrase: "Invoke".into(),
        sigil: "🔮".into(),
        status: ScrollStatus::Draft,
        emotion_signature: EmotionSignature { tone: "neutral".into(), emphasis: 0.5, resonance: "flat".into(), intensity: Some(0.5) },
        linked_scrolls: vec![],
        origin: ScrollOrigin { created: now, authored_by: None, last_modified: now },
    }
}

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
    let scroll = make_scroll(0);
    let high = scorer.score(&inv, &[scroll.clone()], 0.9);
    let low = scorer.score(&inv, &[scroll], 0.1);
    assert!(high > low);
}

#[test]
fn test_semantic_closer_ranked_higher() {
    let scorer = SemanticContextScorer;
    let inv = make_invocation();
    let scroll_recent = make_scroll(0);
    let scroll_old = make_scroll(10);
    let score_recent = scorer.score(&inv, &[scroll_recent], 0.8);
    let score_old = scorer.score(&inv, &[scroll_old], 0.2);
    assert!(score_recent > score_old);
}
