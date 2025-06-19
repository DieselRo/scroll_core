use logtest::Logger;
use uuid::Uuid;
use chrono::Utc;
use scroll_core::archive::archive_memory::InMemoryArchive;
use scroll_core::{EmotionSignature, Scroll, ScrollOrigin, ScrollStatus, ScrollType, YamlMetadata};

fn make_scroll(title: &str, tags: &[&str], body: &str) -> Scroll {
    Scroll {
        id: Uuid::new_v4(),
        title: title.into(),
        scroll_type: ScrollType::Canon,
        yaml_metadata: YamlMetadata {
            title: title.into(),
            scroll_type: ScrollType::Canon,
            emotion_signature: EmotionSignature::neutral(),
            tags: tags.iter().map(|t| t.to_string()).collect(),
            last_modified: None,
            file_path: None,
        },
        markdown_body: body.into(),
        invocation_phrase: "Invoke".into(),
        sigil: "🔮".into(),
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
fn test_semantic_query_returns_relevant_scroll() {
    let mut logger = Logger::start();
    let s1 = make_scroll("Rust Guide", &["rust", "code"], "Learn Rust programming language.");
    let s2 = make_scroll("Cooking Tips", &["cook"], "How to cook pasta.");
    let mut archive = InMemoryArchive::new(vec![s1.clone(), s2.clone()]);
    archive.build_semantic_index();
    let results = archive.query_semantic("rust code tutorial", 1);
    assert_eq!(results.first().unwrap().0.title, "Rust Guide");
    let messages: Vec<String> = logger.map(|r| r.args().to_string()).collect();
    assert!(messages.iter().any(|m| m.contains("Vector generation")));
    assert!(messages.iter().any(|m| m.contains("k-NN search")));
}
