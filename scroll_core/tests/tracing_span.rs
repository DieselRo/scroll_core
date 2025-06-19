use scroll_core::construct_ai::{ConstructAI, ConstructContext, ConstructResult};
use scroll_core::core::construct_registry::ConstructRegistry;
use scroll_core::invocation::invocation_manager::InvocationManager;
use scroll_core::EmotionSignature;

struct Dummy;

impl ConstructAI for Dummy {
    fn reflect_on_scroll(&self, _context: &ConstructContext) -> ConstructResult {
        ConstructResult::Insight {
            text: String::new(),
        }
    }

    fn suggest_scroll(&self, _context: &ConstructContext) -> ConstructResult {
        ConstructResult::Insight {
            text: String::new(),
        }
    }

    fn perform_scroll_action(&self, _context: &ConstructContext) -> ConstructResult {
        ConstructResult::Insight {
            text: String::new(),
        }
    }

    fn name(&self) -> &str {
        "dummy"
    }
}

#[test]
fn test_span_logs() {
    let mut reg = ConstructRegistry::new();
    reg.insert("dummy", Dummy);
    let manager = InvocationManager::new(reg);
    let ctx = ConstructContext {
        scrolls: vec![],
        emotion_signature: EmotionSignature::neutral(),
        tags: vec![],
        user_input: None,
    };
    let _ = manager.invoke_by_name("dummy", &ctx, 0);
}
