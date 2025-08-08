// scroll_core/tests/validator_tests.rs

use chrono::Utc;
use scroll_core::schema::{EmotionSignature, ScrollStatus, ScrollType, YamlMetadata};
use scroll_core::scroll::Scroll;
use scroll_core::validator::{validate_scroll, validate_write_allowed};
use uuid::Uuid;

fn make_scroll_with_status(status: ScrollStatus) -> Scroll {
    let now = Utc::now();
    scroll_core::scroll::Scroll {
        id: Uuid::new_v4(),
        title: "Test".into(),
        scroll_type: ScrollType::Canon,
        yaml_metadata: YamlMetadata {
            title: "Test".into(),
            scroll_type: ScrollType::Canon,
            emotion_signature: EmotionSignature::neutral(),
            tags: vec!["core".into()],
            archetype: None,
            quorum_required: false,
            last_modified: Some(now),
            file_path: None,
        },
        tags: vec!["core".into()],
        archetype: None,
        quorum_required: false,
        markdown_body: String::new(),
        invocation_phrase: String::new(),
        sigil: String::new(),
        status,
        emotion_signature: EmotionSignature::neutral(),
        linked_scrolls: vec![],
        origin: scroll_core::scroll::ScrollOrigin {
            created: now,
            authored_by: None,
            last_modified: now,
        },
    }
}

#[test]
fn metadata_validation_passes() {
    let md = YamlMetadata {
        title: "Valid".into(),
        scroll_type: ScrollType::Canon,
        emotion_signature: EmotionSignature::neutral(),
        tags: vec!["core".into()],
        archetype: None,
        quorum_required: false,
        last_modified: None,
        file_path: None,
    };
    assert!(validate_scroll(&md).is_ok());
}

#[test]
fn metadata_validation_fails_on_empty_title() {
    let md = YamlMetadata {
        title: "   ".into(),
        scroll_type: ScrollType::Canon,
        emotion_signature: EmotionSignature::neutral(),
        tags: vec!["x".into()],
        archetype: None,
        quorum_required: false,
        last_modified: None,
        file_path: None,
    };
    assert!(validate_scroll(&md).is_err());
}

#[test]
fn write_denied_for_sealed() {
    let s = make_scroll_with_status(ScrollStatus::Sealed);
    assert!(validate_write_allowed(&s).is_err());
}

#[test]
fn write_allowed_for_draft() {
    let s = make_scroll_with_status(ScrollStatus::Draft);
    assert!(validate_write_allowed(&s).is_ok());
}
