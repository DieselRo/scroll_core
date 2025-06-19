// ===============================
// src/chat/chat_dispatcher.rs
// ===============================

use crate::chat::chat_router::ChatRouter;
use crate::chat::chat_session::{ChatMessage, ChatSession};
use crate::construct_ai::{ConstructContext, ConstructResult};
use crate::invocation::aelren::AelrenHerald;
use crate::invocation::invocation_manager::InvocationManager;
use crate::schema::EmotionSignature;
use crate::scroll::Scroll;
use crate::trigger_loom::emotional_state::EmotionalState;

pub struct ChatDispatcher;

impl ChatDispatcher {
    pub fn dispatch(
        session: &mut ChatSession,
        user_input: &str,
        manager: &InvocationManager,
        aelren: &AelrenHerald,
        memory: &[Scroll],
        mood: &mut EmotionalState,
    ) -> ChatMessage {
        // Append user message to session
        session.add_message("user", user_input, None);

        let user_msg = session.messages.last().unwrap();
        mood.update_from_message(user_msg);
        let target = ChatRouter::route_target(user_msg)
            .or_else(|| session.target_construct.clone())
            .unwrap_or_else(ChatRouter::default_target);

        // Get latest scroll for context
        let scroll = memory.last().expect("No scrolls available");

        let mut context = manager.registry.build_context(scroll);
        context.user_input = Some(user_input.to_string());

        let result = if target == "symbolic" {
            manager.invoke_symbolically_with_aelren(scroll, aelren)
        } else {
            manager.invoke_by_name(&target, &context, 0)
        };

        let reply = match result {
            ConstructResult::Insight { text } => text,
            ConstructResult::ScrollDraft { content, .. } => content,
            ConstructResult::ModifiedScroll(scroll) => scroll.markdown_body,
            ConstructResult::Refusal { reason, .. } => reason,
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
        mood.update_from_message(&assistant_msg);
        assistant_msg
    }
}
