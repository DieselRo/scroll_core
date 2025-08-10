//! Ambient orchestrator skeleton for Trigger Loom → Invocation wiring.
//!
//! This module provides a minimal facade that can be extended to
//! route ambient ticks into real construct invocations via the
//! `InvocationManager` and ledger. For now it only logs intent.

use crate::invocation::invocation_manager::InvocationManager;
use crate::orchestra::Bus;

#[derive(Default, Clone)]
pub struct AmbientOrchestratorConfig {
    pub max_invocations_per_tick: usize,
}

pub struct AmbientOrchestrator {
    pub bus: Option<Bus>,
    pub config: AmbientOrchestratorConfig,
}

impl AmbientOrchestrator {
    pub fn new() -> Self {
        Self {
            bus: None,
            config: AmbientOrchestratorConfig {
                max_invocations_per_tick: 1,
            },
        }
    }

    pub fn with_bus(mut self, bus: Bus) -> Self {
        self.bus = Some(bus);
        self
    }

    pub fn with_config(mut self, config: AmbientOrchestratorConfig) -> Self {
        self.config = config;
        self
    }

    /// Skeleton: Inspect bus state and, if any ambient triggers are present,
    /// request up to `max_invocations_per_tick` invocations via the manager.
    ///
    /// Currently this is a no-op that returns 0 to indicate no invocations.
    pub fn pump_once(&mut self, _manager: &InvocationManager) -> usize {
        // TODO: examine queued ambient events on the bus and map to constructs
        // via InvocationManager. Log to ledger when wired.
        0
    }
}

