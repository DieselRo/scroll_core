use tracing::dispatcher;

#[test]
fn tracing_initializes() {
    scroll_core::init_tracing_for_test().expect("init tracing");
    assert!(dispatcher::has_been_set());
}
