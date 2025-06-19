use std::sync::Arc;

use chrono::Utc;
use scroll_core::memory::episodic_writer::EpisodicWriterJob;
use scroll_core::sessions::in_memory_session_service::InMemorySessionService;
use scroll_core::sessions::session_service::SessionService;
use scroll_core::events::scroll_event::ScrollEvent;
use scroll_core::models::base_model::LLMResponseContent;

#[tokio::test]
async fn test_episodic_writer_creates_scroll() {
    let service = InMemorySessionService::new();
    let mut session = service
        .create_session("app", "user", None, None)
        .await
        .unwrap();

    for _ in 0..5 {
        let ev = ScrollEvent::new(
            "user".into(),
            Some(LLMResponseContent { text: "hello world".repeat(20) }),
            None,
            false,
            true,
            false,
            None,
        );
        service.append_event(&mut session, ev).await.unwrap();
    }

    let dir = tempfile::tempdir().unwrap();
    let job = EpisodicWriterJob::new(
        Arc::new(service),
        "app".into(),
        "user".into(),
        10,
        dir.path().to_path_buf(),
    );

    job.run_once().await.unwrap();

    let entries: Vec<_> = std::fs::read_dir(&dir).unwrap().collect();
    assert_eq!(entries.len(), 1);
    let content = std::fs::read_to_string(entries[0].as_ref().unwrap().path()).unwrap();
    assert!(content.contains("Conversation"));
    assert!(content.contains("user"));
}
