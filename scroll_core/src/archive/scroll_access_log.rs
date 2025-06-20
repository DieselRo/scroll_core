// scroll_access_log.rs – Tracker of Memory Breath
//==================================================

use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use uuid::Uuid;

/// Tracks how often and recently a scroll has been accessed.
#[derive(Debug, Clone, Serialize)]
pub struct ScrollAccess {
    pub first_accessed: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: usize,
}

impl ScrollAccess {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            first_accessed: now,
            last_accessed: now,
            access_count: 1,
        }
    }

    pub fn record_access(&mut self) {
        self.last_accessed = Utc::now();
        self.access_count += 1;
    }
}

impl Default for ScrollAccess {
    fn default() -> Self {
        Self::new()
    }
}

/// Central memory for tracking scroll access patterns.
pub struct ScrollAccessLog {
    log: HashMap<Uuid, ScrollAccess>,
}

impl ScrollAccessLog {
    pub fn new() -> Self {
        Self {
            log: HashMap::new(),
        }
    }

    /// Records an access to the scroll with the given ID.
    pub fn register_access(&mut self, scroll_id: Uuid) {
        self.log
            .entry(scroll_id)
            .and_modify(|entry| entry.record_access())
            .or_default();
    }

    /// Retrieves access info if it exists.
    pub fn get(&self, scroll_id: &Uuid) -> Option<&ScrollAccess> {
        self.log.get(scroll_id)
    }

    /// Returns number of distinct scrolls tracked.
    pub fn tracked_count(&self) -> usize {
        self.log.len()
    }

    /// Returns the top N most accessed scrolls.
    pub fn most_accessed(&self, top_n: usize) -> Vec<(&Uuid, &ScrollAccess)> {
        let mut entries: Vec<_> = self.log.iter().collect();
        entries.sort_by_key(|(_, access)| usize::MAX - access.access_count);
        entries.into_iter().take(top_n).collect()
    }

    /// Exports the access log as a pretty JSON string.
    pub fn export_log(&self) -> String {
        serde_json::to_string_pretty(&self.log).unwrap_or_else(|_| "{}".to_string())
    }
}

impl Default for ScrollAccessLog {
    fn default() -> Self {
        Self::new()
    }
}
