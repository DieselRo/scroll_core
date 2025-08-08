use scroll_core::core::construct_registry::ConstructRegistry;
use scroll_core::construct_ai::{ConstructAI, ConstructContext, ConstructResult};
use scroll_core::invocation::invocation_manager::InvocationManager;

struct Rejecting; // returns refusal, but manager should still handle
impl ConstructAI for Rejecting {
    fn reflect_on_scroll(&self, _ctx: &ConstructContext) -> ConstructResult { ConstructResult::Refusal { reason: "no".into(), echo: None } }
    fn suggest_scroll(&self, _ctx: &ConstructContext) -> ConstructResult { ConstructResult::Refusal { reason: "no".into(), echo: None } }
    fn perform_scroll_action(&self, _ctx: &ConstructContext) -> ConstructResult { ConstructResult::Refusal { reason: "no".into(), echo: None } }
    fn name(&self) -> &str { "rejector" }
}

#[test]
fn registry_invoke_handles_missing_construct() {
    let reg = ConstructRegistry::new();
    let mgr = InvocationManager::new(reg);
    let ctx = ConstructContext { scrolls: vec![], emotion_signature: Default::default(), tags: vec![], user_input: None };
    let res = mgr.registry.invoke("unknown", &ctx);
    match res { ConstructResult::Refusal { .. } => {}, _ => panic!("expected refusal") }
}


