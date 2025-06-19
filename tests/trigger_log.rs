use logtest::Logger;
use scroll_core::invocation::invocation::{Invocation, InvocationResult};
use scroll_core::invocation::named_construct::{NamedConstruct, PulseSensitive};
use scroll_core::trigger_loom::config::{SymbolicRhythm, TriggerLoopConfig};
use scroll_core::trigger_loom::engine::TriggerLoopEngine;

struct Silent;

impl PulseSensitive for Silent {
    fn should_awaken(&self, _tick: u64) -> bool {
        false
    }
}

impl NamedConstruct for Silent {
    fn name(&self) -> &str {
        "silent"
    }
    fn perform(
        &self,
        _invocation: &Invocation,
        _scroll: Option<scroll_core::Scroll>,
    ) -> Result<InvocationResult, String> {
        Ok(InvocationResult::Success(String::new()))
    }
    fn as_pulse_sensitive(&self) -> Option<&dyn PulseSensitive> {
        Some(self)
    }
}

#[test]
fn test_tick_summary_logging() {
    let logger = Logger::start();
    let config = TriggerLoopConfig {
        rhythm: SymbolicRhythm::Constant(1.0),
        max_invocations_per_tick: 1,
        allow_test_ticks: true,
        emotional_signature: None,
    };
    let mut engine = TriggerLoopEngine::new(config);
    let mut constructs: Vec<Box<dyn NamedConstruct>> = vec![Box::new(Silent)];
    engine.tick_once(&mut constructs);
    let logs = logger.collect();
    let entry = logs.iter().find(|l| l.args.contains("fired")).expect("log");
    let v: serde_json::Value = serde_json::from_str(&entry.args).unwrap();
    assert!(v.get("fired").is_some());
    assert!(v.get("skipped").is_some());
    assert!(v.get("duration_ms").is_some());
}
