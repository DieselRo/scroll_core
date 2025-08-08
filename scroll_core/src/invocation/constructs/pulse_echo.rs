use crate::invocation::named_construct::{NamedConstruct, PulseSensitive};
use crate::invocation::types::{Invocation, InvocationResult};
use crate::orchestra::{AgentMessage, Bus, OrchestratedConstruct};
use serde_json::json;
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct PulseEcho {
    bus: Option<Bus>,
}

impl NamedConstruct for PulseEcho {
    fn name(&self) -> &str { "pulse_echo" }
    fn perform(&self, invocation: &Invocation, _scroll: Option<crate::Scroll>) -> Result<InvocationResult, String> {
        if let Some(bus) = &self.bus {
            let msg = AgentMessage {
                id: Uuid::new_v4(),
                from: "pulse_echo".into(),
                to: "logger".into(),
                payload: json!({"text": format!("pulse:{}", invocation.phrase)}),
                trace: vec!["loom".into(), "pulse".into()],
            };
            bus.send(msg);
        }
        Ok(InvocationResult::Success("ok".into()))
    }
}

impl PulseSensitive for PulseEcho {
    fn should_awaken(&self, tick: u64) -> bool { tick % 2 == 0 }
}

impl OrchestratedConstruct for PulseEcho {
    fn attach_bus(&mut self, bus: Bus) { self.bus = Some(bus); }
}


