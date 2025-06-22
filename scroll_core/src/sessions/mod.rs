//! Core types and services for managing chat sessions and their persistence.
//! Sessions track scroll history and user interaction state.
//! See [Sessions](../../AGENTS.md#contextframeengine) for related constructs.

pub mod database_session_service;
pub mod error;
pub mod in_memory_session_service;
pub mod session;
pub mod session_event_log;
pub mod session_file;
pub mod session_service;
pub mod state;
