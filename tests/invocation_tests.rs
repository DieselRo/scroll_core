
use scroll_core::invocation::constructs::validator_construct::Validator;
use scroll_core::invocation::invocation::{Invocation, InvocationMode, InvocationTier};
use scroll_core::invocation::named_construct::NamedConstruct;
use scroll_core::{Scroll, YamlMetadata, ScrollStatus, ScrollType, EmotionSignature, ScrollOrigin};
use uuid::Uuid;
use chrono::Utc;

#[test]
fn test_validator_construct_validate() {
    let scroll = Scroll {
        id: Uuid::new_v4(),
        title: "Test Scroll".into(),
        scroll_type: ScrollType::Protocol,
        yaml_metadata: YamlMetadata {
            title: "Valid Title".into(),
            scroll_type: ScrollType::Protocol,
            emotion_signature: EmotionSignature::default(),
            tags: vec![],
            last_modified: None,
            file_path: None,
        },
        markdown_body: "This is the body.".into(),
        invocation_phrase: "invoke validator".into(),
        sigil: "🧪".into(),
        status: ScrollStatus::Draft,
        origin: ScrollOrigin {
    created: Utc::now(),
    last_modified: Utc::now(),
},

        emotion_signature: EmotionSignature::default(),
        linked_scrolls: vec![],
    };

    let invocation = Invocation {
        id: Uuid::new_v4(),
        phrase: "Validate this scroll".into(),
        invoker: "TestUser".into(),
        invoked: "Validator".into(),
        tier: InvocationTier::Calling,
        mode: InvocationMode::Validate,
        resonance_required: false,
        timestamp: Utc::now(),
    };

    let validator = Validator;
    let result = validator.perform(&invocation, Some(scroll));
    assert!(matches!(result, Ok(_)));
}
