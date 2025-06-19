use chrono::Utc;
use logtest::Logger;
use tempfile::NamedTempFile;
use std::io::Write;
use std::time::Duration;
use uuid::Uuid;

use scroll_core::archive::archive_memory::InMemoryArchive;
use scroll_core::chat::chat_dispatcher::ChatDispatcher;
use scroll_core::chat::chat_session::ChatSession;
use scroll_core::core::construct_registry::ConstructRegistry;
use scroll_core::core::context_frame_engine::{ContextFrameEngine, ContextMode};
use scroll_core::invocation::aelren::AelrenHerald;
use scroll_core::invocation::invocation_manager::InvocationManager;
use scroll_core::invocation::constructs::validator_construct::Validator;
use scroll_core::invocation::constructs::file_reader_construct::FileReader;
use scroll_core::trigger_loom::emotional_state::EmotionalState;
use scroll_core::{Scroll, ScrollOrigin, ScrollStatus, ScrollType, EmotionSignature, YamlMetadata};

#[test]
fn test_coalition_flow() {
    let logger = Logger::start();
    let mut bad = NamedTempFile::new().unwrap();
    writeln!(bad, "title: :bad").unwrap();

    let mut scroll_file = NamedTempFile::new().unwrap();
    writeln!(scroll_file, "---\ntitle: Test\nscroll_type: protocol\nfile_path: {}\n---\nbody", bad.path().display()).unwrap();

    let mut registry = ConstructRegistry::new();
    registry.insert_orchestrated("validator", Validator::default());
    registry.insert_orchestrated("filereader", FileReader::default());
    let manager = InvocationManager::new(registry);
    let bus = manager.registry.bus();

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

    let rx = bus.subscribe("dispatcher");
    ChatDispatcher::dispatch(&mut session, &format!("@validator check {}", scroll_file.path().display()), &manager, &aelren, &memory, &mut mood);

    let reply = rx.recv_timeout(Duration::from_millis(500)).unwrap();
    assert!(reply.payload["text"].as_str().unwrap().contains("invalid YAML"));
    assert_eq!(reply.trace, ["dispatcher", "validator", "filereader", "validator"]);
    let logs = logger.collect();
    assert!(logs.iter().any(|l| l.args.contains("enqueue to=validator")));
}
