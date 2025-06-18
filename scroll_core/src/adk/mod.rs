// ==================================
// src/adk/mod.rs
// ==================================

//! Agent Development Kit (ADK) for Rust
//! 
//! This module provides a Rust implementation of the Google ADK (Agent Development Kit),
//! which is a framework for building, evaluating, and deploying AI agents.

pub mod agents;
pub mod events;
pub mod models;
pub mod sessions;
pub mod tools;
pub mod artifacts;
pub mod memory;
pub mod common;
pub mod runner;

// Tests
#[cfg(test)]
pub mod tests;

// Re-export commonly used types
pub use common::error::AdkError;
pub use agents::base_agent::BaseAgent;
pub use events::event::Event;
pub use sessions::session::Session;
pub use runner::Runner;