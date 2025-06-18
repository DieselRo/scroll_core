// ===============================
// src/chat/chat_router.rs
// ===============================

use crate::chat::chat_session::ChatMessage;

pub struct ChatRouter;

impl ChatRouter {
    /// Attempt to extract a Construct name from the first word or known patterns.
    pub fn route_target(message: &ChatMessage) -> Option<String> {
        let content = message.content.trim();

        // Match address-style, e.g. "Mythscribe, tell me..."
        if let Some((prefix, _)) = content.split_once(',') {
            return Some(prefix.trim().to_lowercase());
        }

        // Command-style: "/sirion Do we have a schema?"
        if let Some(stripped) = content.strip_prefix('/') {
            if let Some(word) = stripped.split_whitespace().next() {
                return Some(word.trim().to_lowercase());
            }
        }

        None
    }

    /// Fallback construct if routing fails
    pub fn default_target() -> String {
        "mythscribe".into()
    }
}
