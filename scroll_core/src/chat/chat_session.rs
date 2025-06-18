// ===============================
// src/chat/chat_session.rs
// ===============================

use crate::schema::EmotionSignature;

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: String, // "user" | "assistant" | "system"
    pub content: String,
    pub emotion: Option<EmotionSignature>,
}

#[derive(Debug, Clone)]
pub struct ChatSession {
    pub messages: Vec<ChatMessage>,
    pub target_construct: Option<String>,
    pub mood_seed: Option<String>,
}

impl ChatSession {
    pub fn new(target_construct: Option<String>, mood_seed: Option<String>) -> Self {
        ChatSession {
            messages: Vec::new(),
            target_construct,
            mood_seed,
        }
    }

    pub fn add_message(&mut self, role: &str, content: &str, emotion: Option<EmotionSignature>) {
        self.messages.push(ChatMessage {
            role: role.to_string(),
            content: content.to_string(),
            emotion,
        });
    }

    pub fn last_user_message(&self) -> Option<&ChatMessage> {
        self.messages.iter().rev().find(|m| m.role == "user")
    }

    pub fn last_assistant_message(&self) -> Option<&ChatMessage> {
        self.messages.iter().rev().find(|m| m.role == "assistant")
    }
}
