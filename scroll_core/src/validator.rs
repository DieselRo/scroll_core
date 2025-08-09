// ===============================
// src/validator.rs
// ===============================

use crate::schema::{ScrollStatus, ScrollType, YamlMetadata};
use crate::scroll::Scroll;

pub fn validate_scroll(metadata: &YamlMetadata) -> Result<(), String> {
    if metadata.title.trim().is_empty() {
        return Err("Scroll must have a non-empty title.".to_string());
    }

    if matches!(metadata.scroll_type, ScrollType::Myth) && metadata.tags.is_empty() {
        return Err("Myth scrolls must include at least one tag.".to_string());
    }

    if metadata.tags.is_empty() {
        return Err("Scroll must include at least one tag.".to_string());
    }

    if metadata.emotion_signature.tone.trim().is_empty()
        || metadata.emotion_signature.resonance.trim().is_empty()
    {
        return Err("emotion_signature.tone and resonance must be set.".to_string());
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

/// Validate that a write operation is permitted on this scroll.
/// Sealed scrolls cannot be modified via normal write operations.
pub fn validate_write_allowed(scroll: &Scroll) -> Result<(), String> {
    if ScrollStatus::Sealed == scroll.status {
        return Err("Write denied: scroll is sealed.".into());
    }
    Ok(())
}
