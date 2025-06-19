use std::cell::RefCell;
use std::rc::Rc;

use chrono::Utc;
use logtest::Logger;
use scroll_core::core::cost_manager::{self, CostDecision};
use scroll_core::invocation::invocation::{Invocation, InvocationResult};
use scroll_core::invocation::named_construct::{NamedConstruct, PulseSensitive};
use scroll_core::trigger_loom::config::{SymbolicRhythm, TriggerLoopConfig};
use scroll_core::trigger_loom::engine::TriggerLoopEngine;
use uuid::Uuid;

#[derive(Clone)]
struct StubConstruct {
    count: Rc<RefCell<u32>>,
    awaken_every: u64,
}

impl PulseSensitive for StubConstruct {
    fn should_awaken(&self, tick: u64) -> bool {
        tick % self.awaken_every == 0
    }
}

impl NamedConstruct for StubConstruct {
    fn name(&self) -> &str {
        "stub"
    }

    fn perform(
        &self,
        _invocation: &Invocation,
        _scroll: Option<scroll_core::Scroll>,
    ) -> Result<InvocationResult, String> {
        *self.count.borrow_mut() += 1;
        Ok(InvocationResult::Success("ok".into()))
    }

    fn as_pulse_sensitive(&self) -> Option<&dyn PulseSensitive> {
        Some(self)
    }
}

#[test]
fn test_cost_reject_skips_invocation() {
    let logger = Logger::start();
    cost_manager::set_test_decision(Some(CostDecision::Reject("no".into())));
    let config = TriggerLoopConfig {
        rhythm: SymbolicRhythm::Constant(1.0),
        max_invocations_per_tick: 5,
        allow_test_ticks: true,
        emotional_signature: None,
    };
    let mut engine = TriggerLoopEngine::new(config);
    let stub = StubConstruct {
        count: Rc::new(RefCell::new(0)),
        awaken_every: 1,
    };
    let mut constructs: Vec<Box<dyn NamedConstruct>> = vec![Box::new(stub.clone())];
    engine.tick_once(&mut constructs);
    assert_eq!(*stub.count.borrow(), 0);
    let logs = logger.collect();
    assert!(logs.iter().any(|r| r.args.contains("rejected stub")));
    cost_manager::set_test_decision(None);
}

#[test]
fn test_pulse_awaken_every_three() {
    let config = TriggerLoopConfig {
        rhythm: SymbolicRhythm::Constant(1.0),
        max_invocations_per_tick: 5,
        allow_test_ticks: true,
        emotional_signature: None,
    };
    let mut engine = TriggerLoopEngine::new(config);
    let stub = StubConstruct {
        count: Rc::new(RefCell::new(0)),
        awaken_every: 3,
    };
    let silent = StubConstruct {
        count: Rc::new(RefCell::new(0)),
        awaken_every: u64::MAX,
    };
    let mut constructs: Vec<Box<dyn NamedConstruct>> =
        vec![Box::new(stub.clone()), Box::new(silent.clone())];
    for _ in 0..5 {
        engine.tick_once(&mut constructs);
    }
    assert_eq!(*stub.count.borrow(), 1);
    assert_eq!(*silent.count.borrow(), 0);
}

#[test]
fn test_recursion_depth_cap() {
    let config = TriggerLoopConfig {
        rhythm: SymbolicRhythm::Constant(1.0),
        max_invocations_per_tick: 5,
        allow_test_ticks: true,
        emotional_signature: None,
    };
    let mut engine = TriggerLoopEngine::new(config);
    let stub = StubConstruct {
        count: Rc::new(RefCell::new(0)),
        awaken_every: 1,
    };
    let mut constructs: Vec<Box<dyn NamedConstruct>> = vec![Box::new(stub.clone())];
    for _ in 0..10 {
        engine.tick_once(&mut constructs);
    }
    assert!(*stub.count.borrow() <= 5);
}
