use crate::invocation::ledger_service::{LedgerEvent, LedgerHandle};
use crate::invocation::named_construct::{NamedConstruct, PulseSensitive};
use crate::invocation::types::{Invocation, InvocationResult};
use crate::orchestra::{AgentMessage, Bus, OrchestratedConstruct};
use serde_json::json;
use uuid::Uuid;

#[derive(Clone)]
pub struct PulseEcho {
    bus: Option<Bus>,
    ledger: Option<LedgerHandle>,
    /// Whether the echo should awaken at all.
    pub enabled: bool,
    /// Number of ticks between activations.
    pub interval: u64,
}

impl PulseEcho {
    pub fn with_ledger(mut self, ledger: LedgerHandle) -> Self {
        self.ledger = Some(ledger);
        self
    }
}

impl Default for PulseEcho {
    fn default() -> Self {
        Self {
            bus: None,
            ledger: None,
            enabled: true,
            interval: 1,
        }
    }
}

impl NamedConstruct for PulseEcho {
    fn name(&self) -> &str {
        "pulse_echo"
    }
    fn perform(
        &self,
        invocation: &Invocation,
        _scroll: Option<crate::Scroll>,
    ) -> Result<InvocationResult, String> {
        if let Some(bus) = &self.bus {
            let msg = AgentMessage {
                id: Uuid::new_v4(),
                from: "pulse_echo".into(),
                to: "pulse_logger".into(),
                payload: json!({"text": format!("pulse:{}", invocation.phrase)}),
                trace: vec!["loom".into(), "pulse".into()],
            };
            bus.send(msg);
        }
        if let Some(ledger) = &self.ledger {
            let event = LedgerEvent {
                invocation: invocation.clone(),
                cost: crate::core::cost_manager::InvocationCost::default(),
            };
            let _ = ledger.try_log(event);
        }
        Ok(InvocationResult::Success("ok".into()))
    }
}

impl PulseSensitive for PulseEcho {
    fn should_awaken(&self, tick: u64) -> bool {
        self.enabled && tick % self.interval == 0
    }
}

impl OrchestratedConstruct for PulseEcho {
    fn attach_bus(&mut self, bus: Bus) {
        self.bus = Some(bus);
    }
}
