// ===============================
// src/trigger_loom/mod.rs
// ===============================

//! The Trigger Loom orchestrates event-driven constructs.
//!
//! Use [`engine::TriggerLoopEngine`] to register triggers and execute loops.
//!
//! ```rust,no_run
//! use scroll_core::trigger_loom::engine::TriggerLoopEngine;
//! use scroll_core::trigger_loom::config::{TriggerLoopConfig, SymbolicRhythm};
//!
//! let config = TriggerLoopConfig {
//!     rhythm: SymbolicRhythm::Constant(1.0),
//!     max_invocations_per_tick: 10,
//!     allow_test_ticks: true,
//!     emotional_signature: None,
//! };
//! let mut engine = TriggerLoopEngine::new(config);
//! engine.tick_once(&mut []);
//! ```

pub mod config;
pub mod emotion;
pub mod emotional_state;
pub mod engine;
pub mod glyph_matcher;
pub mod loom;
pub mod recursion_control;
pub mod trigger_loop;
