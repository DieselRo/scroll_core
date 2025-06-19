// src/sessions/state.rs
// ===================================================
// Defines the session State system, supporting both
// persistent and temporary (non-committed) deltas.
// ===================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// State holds the full session state: both committed and delta.
/// APP:   Global or app-wide state (prefix: app:)
/// USER:  Per-user memory context (prefix: user:)
/// TEMP:  Ephemeral scratchpad (prefix: temp:)

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct State {
    value: HashMap<String, String>,
    delta: HashMap<String, String>,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_full_map())
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            value: HashMap::new(),
            delta: HashMap::new(),
        }
    }

    pub fn from_parts(base: HashMap<String, String>, delta: HashMap<String, String>) -> Self {
        Self { value: base, delta }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.delta.get(key).or_else(|| self.value.get(key))
    }

    pub fn set(&mut self, key: &str, val: &str) {
        self.delta.insert(key.to_string(), val.to_string());
        self.value.insert(key.to_string(), val.to_string());
    }

    pub fn update(&mut self, updates: HashMap<String, String>) {
        for (k, v) in updates.clone() {
            self.delta.insert(k.clone(), v.clone());
            self.value.insert(k, v);
        }
    }

    pub fn has_delta(&self) -> bool {
        !self.delta.is_empty()
    }

    pub fn to_full_map(&self) -> HashMap<String, String> {
        let mut merged = self.value.clone();
        for (k, v) in &self.delta {
            merged.insert(k.clone(), v.clone());
        }
        merged
    }

    pub fn apply_delta(&mut self, maybe_delta: Option<HashMap<String, String>>) {
        if let Some(delta) = maybe_delta {
            self.update(delta);
        }
    }

    pub fn clear_delta(&mut self) {
        self.delta.clear();
    }

    /// Filters the full merged state by prefix (e.g., "app:", "user:")
    pub fn filter_by_prefix(&self, prefix: &str) -> HashMap<String, String> {
        self.to_full_map()
            .into_iter()
            .filter(|(k, _)| k.starts_with(prefix))
            .collect()
    }
}
