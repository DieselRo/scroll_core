//==========================================
//     src/invocation/invocation_manager.rs
//==========================================

use crate::construct_ai::ConstructResult;
use crate::Scroll;
use crate::core::ConstructRegistry;
use crate::construct_ai::ConstructContext;
use crate::invocation::invocation::InvocationResult;
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
    ) -> InvocationResult {
        if depth > self.max_chain_depth {
            return InvocationResult::Failure("Max invocation depth exceeded".into());
        }

        let result = self.registry.invoke(name, context);
        match result {
            ConstructResult::Success(text) => InvocationResult::Success(text),
            ConstructResult::ModifiedScroll(scroll) => InvocationResult::ModifiedScroll(scroll),
            ConstructResult::Refusal { reason, echo } => {
                InvocationResult::Failure(echo.unwrap_or(reason))
            }
        }
    }

    pub fn invoke_symbolically_with_aelren(
        &self,
        scroll: &Scroll,
        herald: &AelrenHerald,
    ) -> InvocationResult {
        let result = herald.invoke_symbolically(scroll, &self.registry);

        match result {
            ConstructResult::Success(text) => InvocationResult::Success(text),
            ConstructResult::ModifiedScroll(scroll) => InvocationResult::ModifiedScroll(scroll),
            ConstructResult::Refusal { reason, echo } => {
                InvocationResult::Failure(echo.unwrap_or(reason))
            }
        }
    }

    // Optional future batch support
    pub fn invoke_batch(
        &self,
        name: &str,
        contexts: &[ConstructContext],
    ) -> Vec<InvocationResult> {
        contexts
            .iter()
            .map(|ctx| self.invoke_by_name(name, ctx, 0))
            .collect()
    }
}