//! Ambient orchestrator for Trigger Loom → Invocation wiring.
//!
//! The orchestrator acts as a thin bridge between the
//! [`TriggerLoopEngine`](crate::trigger_loom::engine::TriggerLoopEngine)
//! and the [`Bus`](crate::orchestra::Bus). Each call to [`pump_once`]
//! executes a single tick and enqueues demo invocations over the bus so
//! listening constructs can react and log to the ledger.

use crate::invocation::invocation_manager::InvocationManager;
use crate::invocation::named_construct::NamedConstruct;
use crate::orchestra::{AgentMessage, Bus};
use crate::trigger_loom::engine::TriggerLoopEngine;
use serde_json::json;
use uuid::Uuid;

#[derive(Default, Clone)]
pub struct AmbientOrchestratorConfig {
    /// Maximum number of bus activations to enqueue per tick.
    pub max_invocations_per_tick: usize,
    /// When true, CI mode disables non-deterministic demo paths.
    pub ci_mode: bool,
}

pub struct AmbientOrchestrator {
    pub config: AmbientOrchestratorConfig,
}

impl AmbientOrchestrator {
    pub fn new() -> Self {
        Self {
            config: AmbientOrchestratorConfig {
                max_invocations_per_tick: 1,
                ci_mode: false,
            },
        }
    }

    pub fn with_config(mut self, config: AmbientOrchestratorConfig) -> Self {
        self.config = config;
        self
    }

    /// Run a single tick and enqueue demo invocations on the bus.
    ///
    /// Returns the number of activations dispatched over the bus.
    #[allow(clippy::too_many_arguments)]
    pub fn pump_once(
        &mut self,
        _manager: &InvocationManager,
        constructs: &mut [Box<dyn NamedConstruct>],
        engine: &mut TriggerLoopEngine,
        bus: &Bus,
    ) -> usize {
        // Execute one tick. Constructs that are pulse sensitive will perform
        // and log to the invocation ledger via the engine.
        engine.tick_once(constructs);

        // For the Phase 3 demo we simply broadcast a "tick" message to
        // `pulse_logger` each cycle.  In non-CI runs we also notify the
        // `mythscribe_gate` construct to demonstrate optional ambient
        // activations.  These receivers handle ledger logging themselves.
        let mut sent = 0usize;

        if sent < self.config.max_invocations_per_tick {
            let msg = AgentMessage {
                id: Uuid::new_v4(),
                from: "orchestrator".into(),
                to: "pulse_logger".into(),
                payload: json!({"text": "tick"}),
                trace: vec!["loom".into()],
            };
            bus.send(msg);
            sent += 1;
        }

        if !self.config.ci_mode && sent < self.config.max_invocations_per_tick {
            let msg = AgentMessage {
                id: Uuid::new_v4(),
                from: "orchestrator".into(),
                to: "mythscribe_gate".into(),
                payload: json!({"text": "tick"}),
                trace: vec!["loom".into()],
            };
            bus.send(msg);
            sent += 1;
        }

        sent
    }
}
