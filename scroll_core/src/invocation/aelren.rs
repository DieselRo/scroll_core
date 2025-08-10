//! Aelren frames an invocation by gathering context and suggesting which construct should answer.
//! It relies on the ContextFrameEngine and records results to the invocation ledger.
//! See [AelrenHerald](../../AGENTS.md#aelrenherald) for narrative lore.
// src/invocation/aelren.rs

use crate::construct_ai::{ConstructContext, ConstructResult};
use crate::core::context_frame_engine::ContextFrameEngine;
use crate::core::ConstructRegistry;
use crate::invocation::context_ledger::log_context_report;
// DB ledger is written downstream by InvocationManager
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
    pub explain_context: bool,
}

impl<'a> AelrenHerald<'a> {
    pub fn new(frame_engine: ContextFrameEngine<'a>, registry_snapshot: Vec<String>) -> Self {
        Self {
            frame_engine,
            registry_snapshot,
            explain_context: false,
        }
    }

    pub fn frame_invocation(&self, triggering_scroll: &Scroll) -> AelrenFrameResult {
        let (context, report) = self.frame_engine.build_context(triggering_scroll);

        let suggested = self.suggest_construct(&context);
        let echo = if suggested.is_none() {
            Some("The Archive listens, but none may answer yet.".into())
        } else {
            None
        };

        // Persist decisions to ledger (best effort)
        let verbose = self.explain_context;
        let report_clone = report.clone();
        let _ = tokio::runtime::Runtime::new().map(|rt| {
            rt.block_on(async move {
                let _ = log_context_report(&report_clone).await;
            })
        });

        if verbose {
            // Print simple table of decisions
            println!("Context decisions ({}):", report.summary.construct);
            println!("- frame: {}", report.summary.frame_id);
            for d in &report.decisions {
                let status = if d.included { "✔" } else { "✖" };
                let path = d
                    .candidate_path
                    .clone()
                    .unwrap_or_else(|| "<unknown>".into());
                println!(
                    "{} {} | score {:.2} | age {:.1}h | tokens {}/{} | {}",
                    status,
                    path,
                    d.score,
                    d.recency_hours,
                    d.running_tokens,
                    d.max_tokens,
                    d.reason
                );
            }
        }

        let _invocation = Invocation {
            id: Uuid::new_v4(),
            phrase: "Symbolic reflection".into(),
            invoker: "Aelren".into(),
            invoked: suggested.clone().unwrap_or("<none>".into()),
            tier: InvocationTier::Calling,
            mode: InvocationMode::Read,
            resonance_required: false,
            timestamp: Utc::now(),
        };

        // Legacy file logger removed; DB ledger is written by InvocationManager after actual invoke

        AelrenFrameResult {
            framed_context: context,
            suggested_construct: suggested,
            invocation_echo: echo,
        }
    }

    fn suggest_construct(&self, context: &ConstructContext) -> Option<String> {
        // 1) Tag-based hint
        for name in &self.registry_snapshot {
            if context.tags.iter().any(|tag| name.contains(tag)) {
                return Some(name.clone());
            }
        }
        // 2) Tone-based hint (mythscribe for calm/reflective)
        let tone = context.emotion_signature.tone.to_lowercase();
        if let Some(ms) = self
            .registry_snapshot
            .iter()
            .find(|n| n.to_lowercase() == "mythscribe")
        {
            if ["calm", "reflective", "curious", "neutral"]
                .iter()
                .any(|t| tone.contains(t))
            {
                return Some(ms.clone());
            }
        }
        // 3) Default fallback: mythscribe if available, else first construct
        if let Some(ms) = self
            .registry_snapshot
            .iter()
            .find(|n| n.to_lowercase() == "mythscribe")
        {
            return Some(ms.clone());
        }
        self.registry_snapshot.first().cloned()
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
