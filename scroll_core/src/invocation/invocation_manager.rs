//! Central dispatcher responsible for routing invocations to constructs.
//! It consults the AelrenHerald for framing and tracks costs via the CostManager.
//! See [InvocationManager](../../AGENTS.md#invocationmanager) for the council role.
//==========================================
//     src/invocation/invocation_manager.rs
//==========================================

use crate::construct_ai::ConstructContext;
use crate::construct_ai::ConstructResult;
use crate::core::cost_manager::{CostDecision, CostManager, InvocationCost};
use crate::core::ConstructRegistry;
use crate::invocation::aelren::AelrenHerald;
use crate::invocation::ledger_service::{LedgerEvent, LedgerHandle};
use crate::invocation::types::{Invocation, InvocationMode, InvocationTier};
use crate::trigger_loom::ambient;
use chrono::Utc;
use uuid::Uuid;

use crate::Scroll;
use tracing::info_span;

pub struct InvocationManager {
    pub registry: ConstructRegistry,
    pub max_chain_depth: usize,
    pub ledger: Option<LedgerHandle>,
}

impl InvocationManager {
    pub fn new(registry: ConstructRegistry) -> Self {
        Self {
            registry,
            max_chain_depth: 3,
            ledger: None,
        }
    }

    pub fn with_ledger(mut self, handle: LedgerHandle) -> Self {
        self.ledger = Some(handle);
        self
    }

    pub fn invoke_by_name(
        &self,
        name: &str,
        context: &ConstructContext,
        depth: usize,
    ) -> ConstructResult {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
            .try_init();

        if depth > self.max_chain_depth {
            return ConstructResult::Refusal {
                reason: "Max invocation depth exceeded".into(),
                echo: None,
            };
        }
        let invocation = Invocation {
            id: Uuid::new_v4(),
            phrase: "invoke".into(),
            invoker: "InvocationManager".into(),
            invoked: name.to_string(),
            tier: InvocationTier::True,
            mode: InvocationMode::Read,
            resonance_required: false,
            timestamp: Utc::now(),
        };
        let cost = CostManager::assess(&invocation, &context.scrolls).unwrap_or_else(|e| {
            eprintln!("metric error: {e:?}");
            InvocationCost::default()
        });
        // Gate by cost decision before invoking
        match &cost.decision {
            CostDecision::Reject(reason) => {
                return ConstructResult::Refusal {
                    reason: reason.clone(),
                    echo: cost.poetic_rejection.clone(),
                };
            }
            CostDecision::Throttle(_) | CostDecision::Allow => {}
        }
        let system_pressure = cost.cost_profile.system_pressure;
        let token_pressure = cost.cost_profile.token_pressure;
        let _span = info_span!(
            "construct.invoke",
            construct = %name,
            system_pressure = system_pressure,
            token_pressure = token_pressure
        )
        .entered();

        #[cfg(feature = "metrics")]
        let timer = std::time::Instant::now();

        #[cfg(feature = "metrics")]
        {
            let labels = [("construct", name.to_string())];
            metrics::counter!("construct_invocations_total", &labels).increment(1);
        }

        let result = self.registry.invoke(name, context);

        // Non-blocking ledger write via channel; ignore backpressure drops
        if let Some(handle) = &self.ledger {
            let _ = handle.try_log(LedgerEvent {
                invocation: invocation.clone(),
                cost: cost.clone(),
            });
        }

        #[cfg(feature = "metrics")]
        metrics::histogram!("construct_duration_ms").record(timer.elapsed().as_millis() as f64);

        result
    }

    pub fn invoke_symbolically_with_aelren(
        &self,
        scroll: &Scroll,
        herald: &AelrenHerald,
    ) -> ConstructResult {
        // Ambient hook: if tags + emotion would trigger a default action, log it to ledger
        let ambient_hit = ambient::should_trigger(
            &scroll.yaml_metadata.tags,
            &crate::trigger_loom::emotional_state::EmotionalState::new(vec![], 0.8, None),
            "core",
            0.7,
        );
        if ambient_hit {
            let invocation = Invocation {
                id: Uuid::new_v4(),
                phrase: "ambient".into(),
                invoker: "Ambient".into(),
                invoked: "ambient".into(),
                tier: InvocationTier::Calling,
                mode: InvocationMode::Read,
                resonance_required: false,
                timestamp: Utc::now(),
            };
            let cost = InvocationCost::default();
            if let Some(handle) = &self.ledger {
                let _ = handle.try_log(LedgerEvent { invocation, cost });
            }
        }
        herald.invoke_symbolically(scroll, &self.registry)
    }

    // Optional future batch support
    pub fn invoke_batch(&self, name: &str, contexts: &[ConstructContext]) -> Vec<ConstructResult> {
        contexts
            .iter()
            .map(|ctx| self.invoke_by_name(name, ctx, 0))
            .collect()
    }
}
