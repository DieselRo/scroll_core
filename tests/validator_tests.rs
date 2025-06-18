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
    };
    assert!(validate_scroll(&metadata).is_err());
}