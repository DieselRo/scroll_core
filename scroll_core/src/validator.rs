// ===============================
// src/validator.rs
// ===============================

use crate::schema::{ScrollStatus, ScrollType, YamlMetadata};
use crate::scroll::Scroll;

fn validate_only(metadata: &YamlMetadata) -> Result<(), String> {
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

pub fn validate_scroll(metadata: &YamlMetadata) -> Result<(), String> {
    let result = validate_only(metadata);
    if let Some(path) = &metadata.file_path {
        let canon = std::fs::canonicalize(path)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| path.to_string());
        if let Ok(rt) = tokio::runtime::Runtime::new() {
            let fut = async {
                let conn = match crate::sessions::database::ensure_ready_from_env().await {
                    Ok(c) => c,
                    Err(_) => return,
                };
                let ac = crate::threads::thread_autocapture::ThreadAutocapture::new(conn);
                match &result {
                    Ok(_) => {
                        let _ = ac.on_validator_pass(&canon).await;
                    }
                    Err(msg) => {
                        if let Some(rest) = msg.strip_prefix("CONTRADICTION:") {
                            let mut parts = rest.trim().split_whitespace();
                            let code = parts.next();
                            let title = format!("Contradiction: {}", metadata.title);
                            let _ = ac.on_validator_contradiction(&canon, code, &title).await;
                        } else {
                            let title = format!("Validation failed: {}", metadata.title);
                            let _ = ac.on_validator_failure(&canon, &title).await;
                        }
                    }
                }
            };
            let _ = rt.block_on(fut);
        }
    }
    result
}

/// Validate that a write operation is permitted on this scroll.
/// Sealed scrolls cannot be modified via normal write operations.
pub fn validate_write_allowed(scroll: &Scroll) -> Result<(), String> {
    if ScrollStatus::Sealed == scroll.status {
        return Err("Write denied: scroll is sealed.".into());
    }
    Ok(())
}
