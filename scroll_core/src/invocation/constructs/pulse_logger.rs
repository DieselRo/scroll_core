use crate::invocation::named_construct::{NamedConstruct, PulseSensitive};
use crate::invocation::types::{Invocation, InvocationResult};

#[derive(Clone, Default)]
pub struct PulseLogger;

impl NamedConstruct for PulseLogger {
    fn name(&self) -> &str {
        "pulse_logger"
    }
    fn perform(
        &self,
        invocation: &Invocation,
        _scroll: Option<crate::Scroll>,
    ) -> Result<InvocationResult, String> {
        println!("[pulse_logger] tick -> {}", invocation.phrase);
        Ok(InvocationResult::Success("ok".into()))
    }
    fn as_pulse_sensitive(&self) -> Option<&dyn PulseSensitive> {
        Some(self)
    }
}

impl PulseSensitive for PulseLogger {
    fn should_awaken(&self, tick: u64) -> bool {
        tick % 3 == 0
    }
}
