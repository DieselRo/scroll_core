// ===============================
// src/named_construct.rs
// ===============================

use crate::invocation::invocation::{Invocation, InvocationResult};
use crate::Scroll;

pub trait NamedConstruct {
    fn name(&self) -> &str;
    fn perform(&self, invocation: &Invocation, scroll: Option<Scroll>) -> Result<InvocationResult, String>;
}