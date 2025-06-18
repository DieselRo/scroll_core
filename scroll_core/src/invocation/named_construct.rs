// ===============================
// src/invocation/named_construct.rs
// ===============================
#![allow(dead_code)]
#![allow(unused_imports)]


use crate::invocation::invocation::{Invocation, InvocationResult};
use crate::scroll::Scroll;

pub trait NamedConstruct {
    fn name(&self) -> &str;
    fn perform(&self, invocation: &Invocation, scroll: Option<Scroll>) -> Result<InvocationResult, String>;
}