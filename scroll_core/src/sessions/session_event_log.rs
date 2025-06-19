// src/sessions/session_event_log.rs
// ===================================================
// Provides logging of ScrollEvents into session-specific logs.
// Supports persistence and future externalization.
// ===================================================

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::events::ScrollEvent;

/// Simple log structure used to persist session history.
/// In the future this may be replaced with a more advanced logging backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEventLog {
    pub session_id: String,
    pub events: Vec<ScrollEvent>,
    pub created_at: u64,
    pub last_modified: u64,
}

impl SessionEventLog {
    pub fn new(session_id: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            events: vec![],
            created_at: Utc::now().timestamp() as u64,
            last_modified: Utc::now().timestamp() as u64,
        }
    }

    pub fn append_event(&mut self, event: ScrollEvent) {
        self.last_modified = Utc::now().timestamp() as u64;
        self.events.push(event);
    }

    pub fn load_from_file(path: PathBuf) -> Option<Self> {
        if !path.exists() {
            return None;
        }

        let file = File::open(path).ok()?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).ok()
    }

    pub fn save_to_file(&self, path: PathBuf) -> std::io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn export(&self) -> Vec<ScrollEvent> {
        self.events.clone()
    }
}
