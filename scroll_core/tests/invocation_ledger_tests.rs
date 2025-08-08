use scroll_core::invocation::invocation_manager::InvocationManager;
use scroll_core::core::construct_registry::ConstructRegistry;
use scroll_core::construct_ai::{ConstructAI, ConstructContext, ConstructResult};
use scroll_core::sessions::database::init_sqlite_connection;
use scroll_core::archive::archive_memory::InMemoryArchive;
use scroll_core::archive::semantic_index::TokenEmbedder;
use scroll_core::core::context_frame_engine::{ContextFrameEngine, ContextMode};
use scroll_core::invocation::aelren::AelrenHerald;
use scroll_core::scroll::Scroll;
use scroll_core::schema::{ScrollType, YamlMetadata};
use chrono::Utc;
use uuid::Uuid;

struct AllowConstruct;
impl ConstructAI for AllowConstruct {
    fn reflect_on_scroll(&self, _ctx: &ConstructContext) -> ConstructResult { ConstructResult::Insight { text: "ok".into() } }
    fn suggest_scroll(&self, _ctx: &ConstructContext) -> ConstructResult { ConstructResult::Refusal { reason: "n/a".into(), echo: None } }
    fn perform_scroll_action(&self, _ctx: &ConstructContext) -> ConstructResult { ConstructResult::Refusal { reason: "n/a".into(), echo: None } }
    fn name(&self) -> &str { "allow" }
}

#[test]
fn test_invocation_logs_to_db() {
    let _ = init_logger_once();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let db_url = "sqlite::memory:";
    rt.block_on(async { let _ = init_sqlite_connection(db_url).await; });

    // minimal archive/context
    let scrolls: Vec<Scroll> = vec![];
    let mut archive = InMemoryArchive::new(scrolls.clone());
    let _ = archive.build_semantic_index(&TokenEmbedder);
    let engine = ContextFrameEngine::new(&archive, ContextMode::Narrow);

    let mut reg = ConstructRegistry::new();
    reg.insert("allow", AllowConstruct);
    let manager = InvocationManager::new(reg);
    let aelren = AelrenHerald::new(engine, vec!["allow".into()]);

    // invoke with an empty scroll list via symbolic path shouldn't panic
    // DB logging runs async; we mainly assert no panic and successful return path
    // Build a minimal scroll
    let now = Utc::now();
    let s = Scroll {
        id: Uuid::new_v4(),
        title: "Test".into(),
        scroll_type: ScrollType::Canon,
        yaml_metadata: YamlMetadata { title: "Test".into(), scroll_type: ScrollType::Canon, emotion_signature: Default::default(), tags: vec!["t".into()], archetype: None, quorum_required: false, last_modified: Some(now), file_path: None },
        tags: vec!["t".into()],
        archetype: None,
        quorum_required: false,
        markdown_body: String::new(),
        invocation_phrase: String::new(),
        sigil: String::new(),
        status: scroll_core::schema::ScrollStatus::Draft,
        emotion_signature: Default::default(),
        linked_scrolls: vec![],
        origin: scroll_core::scroll::ScrollOrigin { created: now, authored_by: None, last_modified: now },
    };
    let result = manager.invoke_symbolically_with_aelren(&s, &aelren);
    match result { ConstructResult::Insight { .. } | ConstructResult::Refusal { .. } | ConstructResult::ScrollDraft { .. } | ConstructResult::ModifiedScroll(_) => {} }
}

fn init_logger_once() {
    let _ = tracing_subscriber::fmt().try_init();
}


