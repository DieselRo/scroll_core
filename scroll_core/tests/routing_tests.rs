use scroll_core::archive::archive_memory::InMemoryArchive;
use scroll_core::archive::semantic_index::TokenEmbedder;
use scroll_core::construct_ai::{ConstructAI, ConstructContext, ConstructResult};
use scroll_core::core::construct_registry::ConstructRegistry;
use scroll_core::core::context_frame_engine::{ContextFrameEngine, ContextMode};
use scroll_core::invocation::aelren::AelrenHerald;
use scroll_core::Scroll;

struct EchoConstruct(&'static str);
impl ConstructAI for EchoConstruct {
    fn reflect_on_scroll(&self, ctx: &ConstructContext) -> ConstructResult {
        ConstructResult::Insight {
            text: format!("{}:{}", self.0, ctx.emotion_signature.tone),
        }
    }
    fn suggest_scroll(&self, _ctx: &ConstructContext) -> ConstructResult {
        ConstructResult::Refusal {
            reason: "n/a".into(),
            echo: None,
        }
    }
    fn perform_scroll_action(&self, _ctx: &ConstructContext) -> ConstructResult {
        ConstructResult::Refusal {
            reason: "n/a".into(),
            echo: None,
        }
    }
    fn name(&self) -> &str {
        self.0
    }
}

#[test]
fn aelren_uses_tone_and_fallback() {
    let scrolls: Vec<Scroll> = vec![];
    let mut archive = InMemoryArchive::new(scrolls.clone());
    let _ = archive.build_semantic_index(&TokenEmbedder);
    let engine = ContextFrameEngine::new(&archive, ContextMode::Narrow);
    let mut reg = ConstructRegistry::new();
    reg.insert("mythscribe", EchoConstruct("mythscribe"));
    reg.insert("loreweaver", EchoConstruct("loreweaver"));
    let snapshot = reg.list_constructs();
    let aelren = AelrenHerald::new(engine, snapshot);

    // Build a dummy scroll with calm tone
    let mut s = Scroll::builder("Calm Test").build();
    s.emotion_signature = scroll_core::schema::EmotionSignature::reflective();

    // Should route to mythscribe by tone
    if let scroll_core::construct_ai::ConstructResult::Insight { text } =
        aelren.invoke_symbolically(&s, &reg)
    {
        assert!(text.starts_with("mythscribe:"));
    } else {
        panic!("expected insight")
    }
}
