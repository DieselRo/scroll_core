use logtest::Logger;
use scroll_core::archive::archive_memory::InMemoryArchive;
use scroll_core::archive::semantic_index::TokenEmbedder;
use scroll_core::core::context_frame_engine::{ContextFrameEngine, ContextMode};
use scroll_core::Scroll;

#[test]
fn test_context_engine_semantic_recall() {
    let logger = Logger::start();
    let s1 = Scroll::builder("Rust Guide")
        .tags(["rust", "code"].as_ref())
        .body("Learn Rust.")
        .invocation_phrase("Invoke")
        .sigil("🔮")
        .build();
    let s2 = Scroll::builder("Cooking Tips")
        .tags(["cook"].as_ref())
        .body("How to cook pasta.")
        .invocation_phrase("Invoke")
        .sigil("🔮")
        .build();
    let mut archive = InMemoryArchive::new(vec![s1.clone(), s2]);
    archive.build_semantic_index(&TokenEmbedder).unwrap();

    let trigger = Scroll::builder("Advanced Rust patterns")
        .tags(["programming"].as_ref())
        .body("Macros and traits.")
        .invocation_phrase("Invoke")
        .sigil("🔮")
        .build();
    let engine = ContextFrameEngine::new(&archive, ContextMode::Broad);
    let ctx = engine.build_context(&trigger);
    assert!(ctx.scrolls.iter().any(|s| s.title == "Rust Guide"));

    let logs: Vec<String> = logger.map(|r| r.args().to_string()).collect();
    assert!(logs.iter().any(|m| m.contains("k-NN search")));
}
