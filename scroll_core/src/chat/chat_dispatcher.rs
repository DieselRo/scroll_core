// ===============================
// src/chat/chat_dispatcher.rs
// ===============================

use crate::chat::chat_session::{ChatSession, ChatMessage};
use crate::chat::chat_router::ChatRouter;
use crate::construct_ai::{ConstructContext, ConstructResult};
use crate::invocation::invocation_manager::InvocationManager;
use crate::invocation::invocation::InvocationResult;
use crate::invocation::aelren::AelrenHerald;
use crate::scroll::Scroll;
use crate::schema::EmotionSignature;

pub struct ChatDispatcher;

impl ChatDispatcher {
    pub fn dispatch(
        session: &mut ChatSession,
        user_input: &str,
        manager: &InvocationManager,
        aelren: &AelrenHerald,
        memory: &[Scroll],
    ) -> ChatMessage {
        // Append user message to session
        session.add_message("user", user_input, None);

        let user_msg = session.messages.last().unwrap();
        let target = ChatRouter::route_target(user_msg)
            .or_else(|| session.target_construct.clone())
            .unwrap_or_else(ChatRouter::default_target);

        // Get latest scroll for context
        let scroll = memory.last().expect("No scrolls available");

        let context = ConstructContext {
            scrolls: vec![scroll.clone()],
            emotion_signature: EmotionSignature {
                tone: "neutral".into(),
                emphasis: 0.0,
                resonance: "balanced".into(),
                intensity: Some(0.0),
            },
            tags: vec![],
            user_input: Some(user_input.to_string()),
        };

        let result = if target == "symbolic" {
            manager.invoke_symbolically_with_aelren(scroll, aelren)
        } else {
            manager.invoke_by_name(&target, &context, 0)
        };
        
        let reply = match result {
            InvocationResult::Success(text) => text,
            InvocationResult::ModifiedScroll(scroll) => scroll.markdown_body,
            InvocationResult::Failure(reason) => reason,
        };

        let assistant_msg = ChatMessage {
            role: "assistant".into(),
            content: reply,
            emotion: Some(EmotionSignature {
                tone: "reflective".into(),
                emphasis: 0.5,
                resonance: todo!(),
                intensity: Some(0.5),
                
            }),
        };

        session.messages.push(assistant_msg.clone());
        assistant_msg
    }
}
