//==========================================
//     src/invocation/invocation_manager.rs
//==========================================

use crate::construct_ai::ConstructResult;
use crate::Scroll;
use crate::core::ConstructRegistry;
use crate::construct_ai::ConstructContext;
use crate::invocation::aelren::AelrenHerald;

pub struct InvocationManager {
    pub registry: ConstructRegistry,
    pub max_chain_depth: usize,
}

impl InvocationManager {
    pub fn new(registry: ConstructRegistry) -> Self {
        Self {
            registry,
            max_chain_depth: 3,
        }
    }

    pub fn invoke_by_name(
        &self,
        name: &str,
        context: &ConstructContext,
        depth: usize,
    ) -> ConstructResult {
        if depth > self.max_chain_depth {
            return ConstructResult::Refusal { reason: "Max invocation depth exceeded".into(), echo: None };
        }

        self.registry.invoke(name, context)
    }

    pub fn invoke_symbolically_with_aelren(
        &self,
        scroll: &Scroll,
        herald: &AelrenHerald,
    ) -> ConstructResult {
        herald.invoke_symbolically(scroll, &self.registry)
    }

    // Optional future batch support
    pub fn invoke_batch(
        &self,
        name: &str,
        contexts: &[ConstructContext],
    ) -> Vec<ConstructResult> {
        contexts
            .iter()
            .map(|ctx| self.invoke_by_name(name, ctx, 0))
            .collect()
    }
}