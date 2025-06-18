use scroll_core::EmotionSignature;
use scroll_core::{Scroll, ScrollStatus, ScrollOrigin, try_transition};
use uuid::Uuid;
use chrono::Utc;

fn dummy_scroll() -> Scroll {
    Scroll {
        id: Uuid::new_v4(),
        title: "Test".into(),
        scroll_type: scroll_core::ScrollType::Canon,
        yaml_metadata: scroll_core::YamlMetadata {
            title: "Test".into(),
            scroll_type: scroll_core::ScrollType::Canon,
            emotion_signature: EmotionSignature {
                tone: "calm".into(),
                emphasis: 0.5,
                resonance: "gentle".into(),
},
        },
        markdown_body: "Body".into(),
        invocation_phrase: "Invoke".into(),
        sigil: "🔧".into(),
        status: ScrollStatus::Draft,
        emotion_signature: EmotionSignature {
            tone: "calm".into(),
            emphasis: 0.5,
            resonance: "gentle".into(),
},
        linked_scrolls: vec![],
        origin: ScrollOrigin {
            created: Utc::now(),
            last_modified: Utc::now(),
        },
    }
}

#[test]
fn test_valid_transition() {
    let mut scroll = dummy_scroll();
    let result = try_transition(&mut scroll, ScrollStatus::Active);
    assert!(result.is_ok());
    assert_eq!(scroll.status, ScrollStatus::Active);
}

#[test]
fn test_invalid_transition() {
    let mut scroll = dummy_scroll();
    let result = try_transition(&mut scroll, ScrollStatus::Archived);
    assert!(result.is_err());
}