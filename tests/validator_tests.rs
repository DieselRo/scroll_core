use scroll_core::{validate_scroll, ScrollType, YamlMetadata, EmotionSignature};

#[test]
fn test_valid_metadata() {
    let metadata = YamlMetadata {
        title: "Valid Scroll".into(),
        scroll_type: ScrollType::Protocol,
        emotion_signature: EmotionSignature {
    tone: "calm".into(),
    emphasis: 0.5,
    resonance: "gentle".into(),
},
        tags: vec!["test".into()],
        last_modified: None,
        file_path: None,
    };
    assert!(validate_scroll(&metadata).is_ok());
}

#[test]
fn test_empty_title() {
    let metadata = YamlMetadata {
        title: "".into(),
        scroll_type: ScrollType::Protocol,
        emotion_signature: EmotionSignature {
    tone: "calm".into(),
    emphasis: 0.5,
    resonance: "gentle".into(),
},
        tags: vec![],
        last_modified: None,
        file_path: None,
    };
    assert!(validate_scroll(&metadata).is_err());
}

#[test]
fn test_myth_requires_tags() {
    let metadata = YamlMetadata {
        title: "Myth Scroll".into(),
        scroll_type: ScrollType::Myth,
        emotion_signature: EmotionSignature::default(),
        tags: vec![],
        last_modified: None,
        file_path: None,
    };
    assert!(validate_scroll(&metadata).is_err());
}