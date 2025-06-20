// ===============================
// src/chat/chat_dispatcher.rs
// ===============================

use crate::chat::chat_router::ChatRouter;
use crate::chat::chat_session::{ChatMessage, ChatSession};
use crate::construct_ai::ConstructResult;
use crate::core::context_frame_engine::ContextFrameEngine;
use crate::invocation::aelren::AelrenHerald;
use crate::invocation::invocation_core::{Invocation, InvocationMode, InvocationTier};
use crate::invocation::invocation_manager::InvocationManager;
use crate::orchestra::AgentMessage;
use crate::schema::EmotionSignature;
use crate::scroll::Scroll;
use crate::trigger_loom::emotional_state::EmotionalState;
use chrono::Utc;
use log::info;
use uuid::Uuid;

pub struct ChatDispatcher;

impl ChatDispatcher {
    #[allow(deprecated)]
    pub fn new(_manager: &InvocationManager, _engine: &ContextFrameEngine) -> Self {
        ChatDispatcher
    }

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
        let target_opt = ChatRouter::route_target(user_msg);
        let explicit = user_input.contains('@') || user_input.trim_start().starts_with('/');

        if explicit {
            let agent = target_opt.unwrap_or_else(ChatRouter::default_target);
            let mut bus = manager.registry.bus();
            let rx = bus.subscribe("dispatcher");

            info!("enqueue to={}", agent);
            let _invocation = Invocation {
                id: Uuid::new_v4(),
                phrase: user_input.to_string(),
                invoker: "dispatcher".into(),
                invoked: agent.clone(),
                tier: InvocationTier::True,
                mode: InvocationMode::Read,
                resonance_required: false,
                timestamp: Utc::now(),
            };

            let msg = AgentMessage {
                id: Uuid::new_v4(),
                from: "dispatcher".into(),
                to: agent.clone(),
                payload: serde_json::json!({"text": user_input, "invocation": {"invoked": agent.clone()}}),
                trace: vec!["dispatcher".into()],
            };
            bus.send(msg);

            if let Ok(reply) = rx.recv_timeout(std::time::Duration::from_millis(100)) {
                let reply_text = reply.payload["text"].as_str().unwrap_or("").to_string();
                let assistant_msg = ChatMessage {
                    role: "assistant".into(),
                    content: reply_text,
                    emotion: Some(EmotionSignature {
                        tone: "reflective".into(),
                        emphasis: 0.5,
                        resonance: "balanced".into(),
                        intensity: Some(0.5),
                    }),
                };
                session.messages.push(assistant_msg.clone());
                mood.update_from_message(&assistant_msg);
                return assistant_msg;
            }

            let assistant_msg = ChatMessage {
                role: "assistant".into(),
                content: format!("No reply from {}", agent),
                emotion: Some(EmotionSignature {
                    tone: "reflective".into(),
                    emphasis: 0.5,
                    resonance: "balanced".into(),
                    intensity: Some(0.5),
                }),
            };
            session.messages.push(assistant_msg.clone());
            mood.update_from_message(&assistant_msg);
            assistant_msg
        } else {
            let target = session
                .target_construct
                .clone()
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
                    resonance: "balanced".into(),
                    intensity: Some(0.5),
                }),
            };

            session.messages.push(assistant_msg.clone());
            mood.update_from_message(&assistant_msg);
            assistant_msg
        }
    }
}
