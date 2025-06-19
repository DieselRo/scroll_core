use chrono::Utc;
use logtest::Logger;
use scroll_core::archive::archive_memory::InMemoryArchive;
use scroll_core::core::context_frame_engine::{ContextFrameEngine, ContextMode};
use scroll_core::{EmotionSignature, Scroll, ScrollOrigin, ScrollStatus, ScrollType, YamlMetadata};
use uuid::Uuid;

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
fn test_context_engine_semantic_recall() {
    let mut logger = Logger::start();
    let s1 = make_scroll("Rust Guide", &["rust", "code"], "Learn Rust.");
    let s2 = make_scroll("Cooking Tips", &["cook"], "How to cook pasta.");
    let mut archive = InMemoryArchive::new(vec![s1.clone(), s2]);
    archive.build_semantic_index();

    let trigger = make_scroll(
        "Advanced Rust patterns",
        &["programming"],
        "Macros and traits.",
    );
    let engine = ContextFrameEngine::new(&archive, ContextMode::Broad);
    let ctx = engine.build_context(&trigger);
    assert!(ctx.scrolls.iter().any(|s| s.title == "Rust Guide"));

    let logs: Vec<String> = logger.map(|r| r.args().to_string()).collect();
    assert!(logs.iter().any(|m| m.contains("k-NN search")));
}
