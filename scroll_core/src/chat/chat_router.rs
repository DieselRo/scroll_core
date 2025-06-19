// ===============================
// src/chat/chat_router.rs
// ===============================

use crate::chat::chat_session::ChatMessage;
use regex::Regex;

pub struct ChatRouter;

impl ChatRouter {
    /// Attempt to extract a Construct name from the first word or known patterns.
    pub fn route_target(message: &ChatMessage) -> Option<String> {
        let content = message.content.trim();

        let mention_re = Regex::new(r"@([a-zA-Z0-9_]+)").unwrap();
        if let Some(cap) = mention_re.captures(content) {
            return Some(cap[1].to_lowercase());
        }

        let slash_re = Regex::new(r"/(?P<agent>[a-zA-Z0-9_]+)").unwrap();
        if let Some(cap) = slash_re.captures(content) {
            return Some(cap["agent"].to_lowercase());
        }

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
