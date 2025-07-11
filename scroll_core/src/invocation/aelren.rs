//! Aelren frames an invocation by gathering context and suggesting which construct should answer.
//! It relies on the ContextFrameEngine and records results to the invocation ledger.
//! See [AelrenHerald](../../AGENTS.md#aelrenherald) for narrative lore.
// src/invocation/aelren.rs

use crate::construct_ai::{ConstructContext, ConstructResult};
use crate::core::context_frame_engine::ContextFrameEngine;
use crate::core::ConstructRegistry;
use crate::invocation::ledger;
use crate::invocation::types::{Invocation, InvocationMode, InvocationTier};
use crate::scroll::Scroll;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AelrenFrameResult {
    pub framed_context: ConstructContext,
    pub suggested_construct: Option<String>,
    pub invocation_echo: Option<String>,
}

pub struct AelrenHerald<'a> {
    pub frame_engine: ContextFrameEngine<'a>,
    pub registry_snapshot: Vec<String>,
}

impl<'a> AelrenHerald<'a> {
    pub fn new(frame_engine: ContextFrameEngine<'a>, registry_snapshot: Vec<String>) -> Self {
        Self {
            frame_engine,
            registry_snapshot,
        }
    }

    pub fn frame_invocation(&self, triggering_scroll: &Scroll) -> AelrenFrameResult {
        let context = self.frame_engine.build_context(triggering_scroll);

        let suggested = self.suggest_construct(&context);
        let echo = if suggested.is_none() {
            Some("The Archive listens, but none may answer yet.".into())
        } else {
            None
        };

        let invocation = Invocation {
            id: Uuid::new_v4(),
            phrase: "Symbolic reflection".into(),
            invoker: "Aelren".into(),
            invoked: suggested.clone().unwrap_or("<none>".into()),
            tier: InvocationTier::Calling,
            mode: InvocationMode::Read,
            resonance_required: false,
            timestamp: Utc::now(),
        };

        if let Err(e) = ledger::log_invocation("logs/aelren.log", &invocation) {
            eprintln!("⚠️ Failed to log symbolic invocation: {}", e);
        }

        AelrenFrameResult {
            framed_context: context,
            suggested_construct: suggested,
            invocation_echo: echo,
        }
    }

    fn suggest_construct(&self, context: &ConstructContext) -> Option<String> {
        for name in &self.registry_snapshot {
            if context.tags.iter().any(|tag| name.contains(tag)) {
                return Some(name.clone());
            }
        }
        None
    }

    pub fn invoke_symbolically(
        &self,
        triggering_scroll: &Scroll,
        registry: &ConstructRegistry,
    ) -> ConstructResult {
        let framed = self.frame_invocation(triggering_scroll);

        if let Some(name) = framed.suggested_construct {
            registry.invoke(&name, &framed.framed_context)
        } else if let Some(echo) = framed.invocation_echo {
            ConstructResult::Refusal {
                reason: echo,
                echo: Some("No Construct responded symbolically.".into()),
            }
        } else {
            ConstructResult::Refusal {
                reason: "No suitable Construct found.".into(),
                echo: None,
            }
        }
    }
}
