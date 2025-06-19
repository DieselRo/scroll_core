// ===============================
// src/invocation/named_construct.rs
// ===============================
#![allow(unused_imports)]

use crate::invocation::invocation::{Invocation, InvocationResult};
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
