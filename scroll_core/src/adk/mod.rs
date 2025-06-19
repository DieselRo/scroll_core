// ==================================
// src/adk/mod.rs
// ==================================

//! Agent Development Kit (ADK) for Rust
//!
//! This module provides a Rust implementation of the Google ADK (Agent Development Kit),
//! which is a framework for building, evaluating, and deploying AI agents.

pub mod agents;
pub mod artifacts;
pub mod common;
pub mod events;
pub mod memory;
pub mod models;
pub mod runner;
pub mod sessions;
pub mod tools;

// Tests
#[cfg(test)]
pub mod tests;

// Re-export commonly used types
pub use agents::base_agent::BaseAgent;
pub use common::error::AdkError;
pub use events::event::Event;
pub use runner::Runner;
pub use sessions::session::Session;
