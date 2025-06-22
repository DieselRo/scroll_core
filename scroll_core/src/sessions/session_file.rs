//! Helpers for reading and writing session data to JSON files.
//! Used by both database migrations and in-memory services for export.
//! See [Sessions](../../AGENTS.md#contextframeengine) for architecture.
// sessions/session_file.rs
// ===================================================
// File-based utilities to persist sessions and scroll events.
// ===================================================

use std::fs::File;
use std::io::{BufReader, BufWriter};

use crate::sessions::session::ScrollSession;
use serde_json;

use crate::events::ScrollEvent;

/// Save a full session (including state and events) to a JSON file.
pub fn save_session_to_file(session: &ScrollSession, path: &str) -> std::io::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, session)?;
    Ok(())
}

/// Load a full session from a JSON file.
pub fn load_session_from_file(path: &str) -> std::io::Result<ScrollSession> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let session = serde_json::from_reader(reader)?;
    Ok(session)
}

/// Export only the events of a session to a lightweight log.
pub fn export_events_log(session: &ScrollSession, path: &str) -> std::io::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &session.events)?;
    Ok(())
}

/// Load a ScrollEvent log as a standalone list.
pub fn load_events_log(path: &str) -> std::io::Result<Vec<ScrollEvent>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let events = serde_json::from_reader(reader)?;
    Ok(events)
}
