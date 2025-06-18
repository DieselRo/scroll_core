// ===============================
// src/validator.rs
// ===============================

use crate::schema::{YamlMetadata, ScrollType};

pub fn validate_scroll(metadata: &YamlMetadata) -> Result<(), String> {
    if metadata.title.trim().is_empty() {
        return Err("Scroll must have a non-empty title.".to_string());
    }

    match metadata.scroll_type {
        ScrollType::Canon
        | ScrollType::Protocol
        | ScrollType::System
        | ScrollType::Scrollbook
        | ScrollType::AgentCatalog => Ok(()),
    }
}
