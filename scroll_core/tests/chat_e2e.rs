use scroll_core::archive::archive_memory::InMemoryArchive;
use scroll_core::archive::initialize::load_with_cache;
use scroll_core::archive::scroll_access_log::ScrollAccessLog;
use scroll_core::chat::chat_dispatcher::ChatDispatcher;
use scroll_core::chat::chat_session::ChatSession;
use scroll_core::core::construct_registry::ConstructRegistry;
use scroll_core::core::context_frame_engine::{ContextFrameEngine, ContextMode};
use scroll_core::invocation::aelren::AelrenHerald;
use scroll_core::invocation::constructs::openai_construct::{Mythscribe, OpenAIClient};
use scroll_core::invocation::invocation_manager::InvocationManager;
use scroll_core::trigger_loom::emotional_state::EmotionalState;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test(flavor = "multi_thread")]
async fn test_chat_dispatcher_records_access() {
    // Start mock OpenAI server
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "choices": [{"message": {"content": "Mock insight"}}]
        })))
        .mount(&server)
        .await;

    // Load example scrolls
    let (scrolls, _cache) = load_with_cache("../tests/e2e_scrolls").unwrap();

    let archive = Box::leak(Box::new(InMemoryArchive::new(scrolls.clone())));
    let mut access_log = ScrollAccessLog::new();
    let engine = ContextFrameEngine::new(archive, ContextMode::Narrow);
    let aelren = Box::leak(Box::new(AelrenHerald::new(
        engine,
        vec!["mythscribe".into()],
    )));

    let mut registry = ConstructRegistry::new();
    let client = OpenAIClient {
        api_key: "test".into(),
        model: "gpt-4o".into(),
        endpoint: format!("{}/v1/chat/completions", server.uri()),
        max_tokens: 50,
    };
    registry.insert("mythscribe", Mythscribe::new(client, "System".into()));
    let manager = Box::leak(Box::new(InvocationManager::new(registry)));

    let scrolls_thread = scrolls.clone();
    let reply = tokio::task::spawn_blocking(move || {
        let mut session = ChatSession::new(None, None);
        let mut mood = EmotionalState::new(Vec::new(), 0.0, None);
        ChatDispatcher::dispatch(
            &mut session,
            "mythscribe, speak",
            &manager,
            &aelren,
            &scrolls_thread,
            &mut mood,
        )
    })
    .await
    .unwrap();

    assert_eq!(reply.content, "Mock insight");

    let last_id = scrolls.last().unwrap().id;
    access_log.register_access(last_id);
    assert!(access_log.get(&last_id).is_some());
}
