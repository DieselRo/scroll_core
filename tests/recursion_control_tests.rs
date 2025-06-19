use chrono::Utc;
use scroll_core::invocation::invocation::{Invocation, InvocationMode, InvocationTier};
use scroll_core::trigger_loom::recursion_control::{mark_cycle, recover_trace};
use uuid::Uuid;

#[test]
fn test_recover_last_cycle() {
    let path = std::env::temp_dir().join("recursion_trace.log");
    let _ = std::fs::remove_file(&path);
    let inv1 = Invocation {
        id: Uuid::new_v4(),
        phrase: "a".into(),
        invoker: "x".into(),
        invoked: "y".into(),
        tier: InvocationTier::Calling,
        mode: InvocationMode::Read,
        resonance_required: false,
        timestamp: Utc::now(),
    };
    let inv2 = Invocation {
        id: Uuid::new_v4(),
        phrase: "b".into(),
        invoker: "x".into(),
        invoked: "y".into(),
        tier: InvocationTier::Calling,
        mode: InvocationMode::Read,
        resonance_required: false,
        timestamp: Utc::now(),
    };
    mark_cycle(&inv1);
    mark_cycle(&inv2);
    let last = recover_trace().unwrap();
    assert_eq!(last, inv2.id.to_string());
    let _ = std::fs::remove_file(&path);
}
