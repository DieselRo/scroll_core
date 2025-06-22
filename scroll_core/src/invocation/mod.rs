//! Entry point for the invocation subsystem.
//! It exposes all constructs and routing utilities used during a session.
//! Refer to [InvocationManager](../AGENTS.md#invocationmanager) for the main orchestrator.
// ===============================
// src/invocation/mod.rs
// ===============================

pub mod aelren;
pub mod constructs;
pub mod invocation_manager;
pub mod ledger;
pub mod named_construct;
pub mod types;
