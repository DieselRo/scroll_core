#[cfg(feature = "metrics")]
use metrics_exporter_prometheus::PrometheusBuilder;
#[cfg(feature = "metrics")]
use scroll_core::archive::archive_memory::InMemoryArchive;
#[cfg(feature = "metrics")]
use scroll_core::{EmotionSignature, Scroll, ScrollOrigin, ScrollStatus, ScrollType, YamlMetadata};
#[cfg(feature = "metrics")]
use uuid::Uuid;

#[cfg(feature = "metrics")]
fn make_scroll(title: &str) -> Scroll {
    Scroll {
        id: Uuid::new_v4(),
        title: title.into(),
        scroll_type: ScrollType::Canon,
        yaml_metadata: YamlMetadata {
            title: title.into(),
            scroll_type: ScrollType::Canon,
            emotion_signature: EmotionSignature::default(),
            tags: vec!["test".into()],
            last_modified: None,
            file_path: None,
        },
        markdown_body: "Body".into(),
        invocation_phrase: String::new(),
        sigil: String::new(),
        status: ScrollStatus::Draft,
        emotion_signature: EmotionSignature::default(),
        linked_scrolls: vec![],
        origin: ScrollOrigin {
            created: chrono::Utc::now(),
            authored_by: None,
            last_modified: chrono::Utc::now(),
        },
    }
}

#[cfg(feature = "metrics")]
#[test]
fn test_vector_metrics_recorded() {
    let handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("install recorder");

    let scrolls = vec![make_scroll("one"), make_scroll("two")];
    let mut archive = InMemoryArchive::new(scrolls);
    archive.build_semantic_index();

    handle.run_upkeep();
    let body = handle.render();
    assert!(body.contains("scroll_embed_time_seconds"));
    assert!(body.contains("vector_index_update_time_seconds"));
    assert!(body.contains("vector_index_memory_bytes"));
}
