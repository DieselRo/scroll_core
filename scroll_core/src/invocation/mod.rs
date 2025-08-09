//! Entry point for the invocation subsystem.
//! It exposes all constructs and routing utilities used during a session.
//! Refer to [InvocationManager](../AGENTS.md#invocationmanager) for the main orchestrator.
// ===============================
// src/invocation/mod.rs
// ===============================

pub mod aelren;
pub mod constructs;
pub mod invocation_manager;
pub mod ledger;
pub mod ledger_service;
pub mod llm;
pub mod named_construct;
pub mod types;

/// Adapter to bridge legacy `NamedConstruct` to the newer `ConstructAI` API.
pub mod adapters {
    use crate::construct_ai::{ConstructAI, ConstructContext, ConstructResult};
    use crate::invocation::named_construct::NamedConstruct as LegacyNamedConstruct;
    use crate::invocation::types::{Invocation, InvocationMode};

    pub struct NamedToAIAdapter<T: LegacyNamedConstruct> {
        pub inner: T,
    }

    impl<T: LegacyNamedConstruct> ConstructAI for NamedToAIAdapter<T> {
        fn reflect_on_scroll(&self, context: &ConstructContext) -> ConstructResult {
            let inv = Invocation {
                id: uuid::Uuid::new_v4(),
                phrase: context.user_input.clone().unwrap_or_default(),
                invoker: "adapter".into(),
                invoked: self.inner.name().into(),
                tier: crate::invocation::types::InvocationTier::Calling,
                mode: InvocationMode::Read,
                resonance_required: false,
                timestamp: chrono::Utc::now(),
            };
            let scroll = context.scrolls.first().cloned();
            match self.inner.perform(&inv, scroll) {
                Ok(r) => match r {
                    crate::invocation::types::InvocationResult::Success(text) => {
                        ConstructResult::Insight { text: text.into() }
                    }
                    crate::invocation::types::InvocationResult::ModifiedScroll(s) => {
                        ConstructResult::ModifiedScroll(s)
                    }
                    crate::invocation::types::InvocationResult::Failure(reason) => {
                        ConstructResult::Refusal {
                            reason: reason.into(),
                            echo: None,
                        }
                    }
                },
                Err(e) => ConstructResult::Refusal {
                    reason: e,
                    echo: None,
                },
            }
        }

        fn suggest_scroll(&self, _context: &ConstructContext) -> ConstructResult {
            ConstructResult::Refusal {
                reason: "Not implemented in adapter".into(),
                echo: None,
            }
        }

        fn perform_scroll_action(&self, _context: &ConstructContext) -> ConstructResult {
            ConstructResult::Refusal {
                reason: "Not implemented in adapter".into(),
                echo: None,
            }
        }

        fn name(&self) -> &str {
            self.inner.name()
        }
    }
}
