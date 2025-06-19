use chrono::Utc;
use scroll_core::archive::archive_memory::InMemoryArchive;
use scroll_core::archive::semantic_index::TokenEmbedder;
use scroll_core::{EmotionSignature, Scroll, ScrollOrigin, ScrollStatus, ScrollType, YamlMetadata};
use uuid::Uuid;

fn make_scroll(title: &str) -> Scroll {
    Scroll {
        id: Uuid::new_v4(),
        title: title.into(),
        scroll_type: ScrollType::Canon,
        yaml_metadata: YamlMetadata {
            title: title.into(),
            scroll_type: ScrollType::Canon,
            emotion_signature: EmotionSignature::neutral(),
            tags: vec![],
            archetype: None,
            quorum_required: false,
            last_modified: None,
            file_path: None,
        },
        tags: vec![],
        archetype: None,
        quorum_required: false,
        markdown_body: "body".into(),
        invocation_phrase: String::new(),
        sigil: String::new(),
        status: ScrollStatus::Draft,
        emotion_signature: EmotionSignature::neutral(),
        linked_scrolls: vec![],
        origin: ScrollOrigin {
            created: Utc::now(),
            authored_by: None,
            last_modified: Utc::now(),
        },
    }
}

#[test]
fn builds_index_for_all_scrolls() {
    let scrolls = vec![make_scroll("one"), make_scroll("two")];
    let mut archive = InMemoryArchive::new(scrolls);
    archive.build_semantic_index(&TokenEmbedder).unwrap();
    assert_eq!(archive.semantic_index_len(), 2);
}
