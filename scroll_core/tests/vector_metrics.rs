#[cfg(feature = "metrics")]
use metrics_exporter_prometheus::PrometheusBuilder;
#[cfg(feature = "metrics")]
use scroll_core::archive::archive_memory::InMemoryArchive;
#[cfg(feature = "metrics")]
use scroll_core::archive::semantic_index::TokenEmbedder;
#[cfg(feature = "metrics")]
use scroll_core::Scroll;

#[cfg(feature = "metrics")]
#[test]
fn test_vector_metrics_recorded() {
    let handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("install recorder");

    let scrolls = vec![
        Scroll::builder("one")
            .tags(["test"].as_ref())
            .body("Body")
            .build(),
        Scroll::builder("two")
            .tags(["test"].as_ref())
            .body("Body")
            .build(),
    ];
    let mut archive = InMemoryArchive::new(scrolls);
    archive.build_semantic_index(&TokenEmbedder).unwrap();

    handle.run_upkeep();
    let body = handle.render();
    assert!(body.contains("scroll_embed_time_seconds"));
    assert!(body.contains("vector_index_update_time_seconds"));
    assert!(body.contains("vector_index_memory_bytes"));
}
