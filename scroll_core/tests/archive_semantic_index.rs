use scroll_core::archive::archive_memory::InMemoryArchive;
use scroll_core::archive::semantic_index::TokenEmbedder;
use scroll_core::Scroll;

#[test]
fn builds_index_for_all_scrolls() {
    let scrolls = vec![
        Scroll::builder("one").body("body").build(),
        Scroll::builder("two").body("body").build(),
    ];
    let mut archive = InMemoryArchive::new(scrolls);
    archive.build_semantic_index(&TokenEmbedder).unwrap();
    assert_eq!(archive.semantic_index_len(), 2);
}
