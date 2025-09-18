use crate::invocation::ledger_service::{LedgerEvent, LedgerHandle};
use crate::invocation::named_construct::NamedConstruct;
use crate::invocation::types::{Invocation, InvocationMode, InvocationResult, InvocationTier};
use crate::orchestra::{Bus, OrchestratedConstruct};
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct MythscribeGate {
    bus: Option<Bus>,
    ledger: Option<LedgerHandle>,
    pub ci_mode: bool,
}

impl MythscribeGate {
    pub fn with_ledger(mut self, ledger: LedgerHandle) -> Self {
        self.ledger = Some(ledger);
        self
    }
}

impl NamedConstruct for MythscribeGate {
    fn name(&self) -> &str {
        "mythscribe_gate"
    }

    fn perform(
        &self,
        invocation: &Invocation,
        _scroll: Option<crate::Scroll>,
    ) -> Result<InvocationResult, String> {
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

impl OrchestratedConstruct for MythscribeGate {
    fn attach_bus(&mut self, mut bus: Bus) {
        let rx = bus.subscribe("mythscribe_gate");
        let ledger = self.ledger.clone();
        let ci = self.ci_mode;
        std::thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                if ci {
                    if let Some(ledger) = &ledger {
                        let invocation = Invocation {
                            id: Uuid::new_v4(),
                            phrase: msg
                                .payload
                                .get("text")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            invoker: msg.from.clone(),
                            invoked: "mythscribe_gate".into(),
                            tier: InvocationTier::True,
                            mode: InvocationMode::Read,
                            resonance_required: false,
                            timestamp: Utc::now(),
                        };
                        let event = LedgerEvent {
                            invocation,
                            cost: crate::core::cost_manager::InvocationCost::default(),
                        };
                        let _ = ledger.try_log(event);
                    }
                } else {
                    println!("[mythscribe_gate] demo invoke {:?}", msg.payload);
                }
            }
        });
        self.bus = Some(bus);
    }
}
