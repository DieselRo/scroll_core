// ===============================
// src/validator.rs
// ===============================

use crate::schema::{YamlMetadata, ScrollType};

pub fn validate_scroll(metadata: &YamlMetadata) -> Result<(), String> {
    if metadata.title.trim().is_empty() {
        return Err("Scroll must have a non-empty title.".to_string());
    }

    if matches!(metadata.scroll_type, ScrollType::Myth) && metadata.tags.is_empty() {
        return Err("Myth scrolls must include at least one tag.".to_string());
    }

    match metadata.scroll_type {
        ScrollType::Canon
        | ScrollType::Protocol
        | ScrollType::System
        | ScrollType::Scrollbook
        | ScrollType::AgentCatalog
        | ScrollType::Myth
        | ScrollType::Echo
        | ScrollType::Ritual => Ok(()),
    }
}
