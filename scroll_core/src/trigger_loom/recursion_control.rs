// ===============================
// src/trigger_loom/recursion_control.rs
// ===============================

use crate::invocation::invocation::{Invocation, InvocationTier};

pub fn should_recurse(tier: &InvocationTier) -> bool {
    matches!(tier, InvocationTier::Calling | InvocationTier::Whispered)
}

pub fn mark_cycle(invocation: &Invocation) {
    // Future: Log to scroll echo trace or recursion cycle ledger
    println!("🔁 Recursion marked: {}", invocation.phrase);
}

pub fn recover_trace() -> Option<String> {
    // Placeholder for restoring recursion state
    Some("Recovered prior recursion trace.".to_string())
}