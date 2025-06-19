use logtest::Logger;
use scroll_core::chat::chat_dispatcher::ChatDispatcher;
use scroll_core::chat::chat_session::ChatSession;
use scroll_core::core::construct_registry::ConstructRegistry;
use scroll_core::invocation::aelren::AelrenHerald;
use scroll_core::invocation::invocation_manager::InvocationManager;
use scroll_core::core::context_frame_engine::{ContextFrameEngine, ContextMode};
use scroll_core::archive::archive_memory::InMemoryArchive;
use scroll_core::trigger_loom::emotional_state::EmotionalState;
use scroll_core::{Scroll, ScrollOrigin, ScrollStatus, ScrollType, EmotionSignature, YamlMetadata};
use uuid::Uuid;
use chrono::Utc;

#[test]
fn test_dispatcher_enqueues() {
    let logger = Logger::start();
    let mut registry = ConstructRegistry::new();
    let manager = InvocationManager::new(registry);
    let bus = manager.registry.bus();
    let rx = bus.subscribe("validator");

    let archive = InMemoryArchive::new(Vec::new());
    let engine = ContextFrameEngine::new(&archive, ContextMode::Narrow);
    let aelren = AelrenHerald::new(engine, vec![]);

    let mut session = ChatSession::new(None, None);
    let mut mood = EmotionalState::new(Vec::new(), 0.0, None);
    let scroll = Scroll {
        id: Uuid::new_v4(),
        title: "t".into(),
        scroll_type: ScrollType::Echo,
        yaml_metadata: YamlMetadata {
            title: "t".into(),
            scroll_type: ScrollType::Echo,
            emotion_signature: EmotionSignature::default(),
            tags: vec![],
            last_modified: None,
            file_path: None,
        },
        markdown_body: String::new(),
        invocation_phrase: String::new(),
        sigil: String::new(),
        status: ScrollStatus::Draft,
        emotion_signature: EmotionSignature::default(),
        linked_scrolls: vec![],
        origin: ScrollOrigin { created: Utc::now(), last_modified: Utc::now(), authored_by: None },
    };
    let memory = vec![scroll];

    ChatDispatcher::dispatch(&mut session, "@validator please check scroll 42", &manager, &aelren, &memory, &mut mood);

    rx.recv_timeout(std::time::Duration::from_millis(50)).expect("no message");
    let logs = logger.collect();
    assert!(logs.iter().any(|r| r.args.contains("enqueue to=validator")));
}
