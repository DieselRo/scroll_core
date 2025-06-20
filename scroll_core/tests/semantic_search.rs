use logtest::Logger;
use scroll_core::archive::archive_memory::InMemoryArchive;
use scroll_core::archive::semantic_index::TokenEmbedder;
use scroll_core::Scroll;

#[test]
fn test_semantic_query_returns_relevant_scroll() {
    let logger = Logger::start();
    let s1 = Scroll::builder("Rust Guide")
        .tags(["rust", "code"].as_ref())
        .body("Learn Rust programming language.")
        .invocation_phrase("Invoke")
        .sigil("🔮")
        .build();
    let s2 = Scroll::builder("Cooking Tips")
        .tags(["cook"].as_ref())
        .body("How to cook pasta.")
        .invocation_phrase("Invoke")
        .sigil("🔮")
        .build();
    let mut archive = InMemoryArchive::new(vec![s1.clone(), s2.clone()]);
    archive.build_semantic_index(&TokenEmbedder).unwrap();
    let results = archive.query_semantic("rust code tutorial", 1);
    assert_eq!(results.first().unwrap().0.title, "Rust Guide");
    let messages: Vec<String> = logger.map(|r| r.args().to_string()).collect();
    assert!(messages.iter().any(|m| m.contains("Vector generation")));
    assert!(messages.iter().any(|m| m.contains("k-NN search")));
}
