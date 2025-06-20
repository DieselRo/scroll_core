// ===============================
// src/invocation/construct_invoke.rs
// ===============================

use crate::construct_ai::{ConstructAI, ConstructInsight};
use crate::core::construct_registry::ConstructRegistry;
use crate::scroll::Scroll;
use crate::invocation::{InvocationResult, InvocationMode};
use crate::ledger; // placeholder for Thiren logging

pub struct InvocationRequest {
    pub construct_name: String,
    pub scroll: Scroll,
    pub mode: InvocationMode,
    pub invoker: String,
}

pub struct ConstructInvoker<'a> {
    pub registry: &'a ConstructRegistry,
}

impl<'a> ConstructInvoker<'a> {
    pub fn new(registry: &'a ConstructRegistry) -> Self {
        Self { registry }
    }

    pub fn dispatch(&self, request: InvocationRequest) -> InvocationResult {
        let name = &request.construct_name;
        let mode = &request.mode;
        let mut scroll = request.scroll.clone();

        if let Some(construct) = self.registry.get(name) {
            let result = match mode {
                InvocationMode::Read => {
                    let insight = construct.reflect_on_scroll(&scroll);
                    InvocationResult::Success(insight.summary)
                },
                InvocationMode::Modify => {
                    match construct.perform_scroll_action(&mut scroll) {
                        Ok(updated) => InvocationResult::ModifiedScroll(Box::new(updated)),
                        Err(reason) => InvocationResult::Failure(format!("{} failed: {}", name, reason))
                    }
                },
                InvocationMode::Validate => {
                    let insight = construct.reflect_on_scroll(&scroll);
                    if insight.symbolic_echo.is_some() {
                        InvocationResult::Success(insight.summary)
                    } else {
                        InvocationResult::Failure("Validation failed or unclear.".into())
                    }
                },
                InvocationMode::Custom(task) => {
                    InvocationResult::Success(format!("{} received a custom task: {}", name, task))
                },
                InvocationMode::Transition => {
                    InvocationResult::Success("Transition mode not yet implemented.".into())
                },
            };

            // Log invocation (symbolic placeholder)
            let _ = ledger::log_invocation_trace(name, &request.invoker, &request.mode);

            result
        } else {
            InvocationResult::Failure(format!("No Construct named '{}' found.", name))
        }
    }
}

// 🌀 Placeholder logging helper (to match `Thiren` pattern)
pub mod ledger {
    use crate::invocation::InvocationMode;

    pub fn log_invocation_trace(construct: &str, invoker: &str, mode: &InvocationMode) -> std::io::Result<()> {
        println!("[TRACE] {} invoked {} with mode {:?}", invoker, construct, mode);
        Ok(())
    }
}