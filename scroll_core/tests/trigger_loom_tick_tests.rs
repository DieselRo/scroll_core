use scroll_core::trigger_loom::config::{SymbolicRhythm, TriggerLoopConfig};
use scroll_core::trigger_loom::engine::TriggerLoopEngine;

#[test]
fn trigger_loop_ticks_without_constructs() {
    let cfg = TriggerLoopConfig {
        rhythm: SymbolicRhythm::Constant(2.0),
        max_invocations_per_tick: 1,
        allow_test_ticks: true,
        emotional_signature: None,
    };
    let mut engine = TriggerLoopEngine::new(cfg);
    let mut empty: Vec<Box<dyn scroll_core::invocation::named_construct::NamedConstruct>> = vec![];
    engine.tick_once(&mut empty);
}
