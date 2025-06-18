
use scroll_core::events::scroll_event::ScrollEvent;
use scroll_core::models::base_model::LLMResponseContent;
use scroll_core::sessions::in_memory_session_service::InMemorySessionService;
use scroll_core::sessions::session_service::SessionService;

#[test]
fn test_create_and_append_event() {
    let service = InMemorySessionService::new();

    // Step 1: Create session
    let mut session = service
        .create_session("test_app", "user_123", None, None)
        .unwrap();

    // Step 2: Create a test event
    let test_event = ScrollEvent {
        id: uuid::Uuid::new_v4(),
        author: "test_construct".to_string(),
        timestamp: 0.0,
        content: Some(LLMResponseContent {
            text: "Hello, world!".to_string(),
        }),
        actions: None,
        partial: false,
        turn_complete: true,
        interrupted: false,
        branch: None,
    };

    // Step 3: Append event
    let appended = service.append_event(&mut session, test_event.clone()).unwrap();

    // Step 4: Assert correctness
    assert_eq!(session.events.len(), 1);
    assert_eq!(
        session.events[0].content.as_ref().unwrap().text,
        "Hello, world!"
    );
    assert_eq!(session.events[0].author, "test_construct");
   assert_eq!(appended.content.unwrap().text, "Hello, world!");

}
