//! Traits that all named constructs implement.
//! PulseSensitive constructs can activate on timed loops, while NamedConstruct defines the invocation API.
//! See the directory in [AGENTS](../../AGENTS.md) for implemented constructs.
// ===============================
// src/invocation/named_construct.rs
// ===============================

use crate::invocation::types::{Invocation, InvocationResult};
use crate::scroll::Scroll;

pub trait PulseSensitive {
    fn should_awaken(&self, tick: u64) -> bool;
}

pub trait NamedConstruct {
    fn name(&self) -> &str;
    fn perform(
        &self,
        invocation: &Invocation,
        scroll: Option<Scroll>,
    ) -> Result<InvocationResult, String>;
    fn as_pulse_sensitive(&self) -> Option<&dyn PulseSensitive> {
        None
    }
}
