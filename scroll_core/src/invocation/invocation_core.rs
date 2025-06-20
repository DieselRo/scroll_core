// ===============================
// src/invocation.rs
// ===============================
#![allow(unused_imports)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum InvocationTier {
    True,
    Calling,
    Whispered,
    Faded,
    Sealed,
}

#[derive(Debug, Clone)]
pub enum InvocationMode {
    Read,
    Modify,
    Validate,
    Transition,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct Invocation {
    pub id: Uuid,
    pub phrase: String,
    pub invoker: String, // Can be an AgentId or Name
    pub invoked: String, // Name of the construct
    pub tier: InvocationTier,
    pub mode: InvocationMode,
    pub resonance_required: bool,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum InvocationResult {
    Success(String),
    ModifiedScroll(Box<crate::Scroll>),
    Failure(String),
}
