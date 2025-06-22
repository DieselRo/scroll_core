// ===============================
// src/chat/chat_dispatcher.rs
// ===============================

use crate::chat::chat_router::ChatRouter;
use crate::chat::chat_session::{ChatMessage, ChatSession};
use crate::construct_ai::ConstructResult;
use crate::core::context_frame_engine::ContextFrameEngine;
use crate::invocation::aelren::AelrenHerald;
use crate::invocation::invocation_manager::InvocationManager;
use crate::invocation::types::{Invocation, InvocationMode, InvocationTier};
use crate::orchestra::AgentMessage;
use crate::schema::EmotionSignature;
use crate::scroll::Scroll;
use crate::trigger_loom::emotional_state::EmotionalState;
use atty::Stream;
use chrono::Utc;
use clap::{arg, Command};
use log::info;
use std::io::{BufRead, Write};
use std::process::{Command as ProcessCommand, Stdio};
use uuid::Uuid;

pub struct ChatDispatcher;

impl ChatDispatcher {
    #[allow(deprecated)]
    pub fn new(_manager: &InvocationManager, _engine: &ContextFrameEngine) -> Self {
        ChatDispatcher
    }

    fn system_msg(text: String) -> ChatMessage {
        ChatMessage {
            role: "system".into(),
            content: text,
            emotion: None,
        }
    }

    fn pager_display(text: &str) -> std::io::Result<()> {
        if atty::is(Stream::Stdout) {
            let pager = std::env::var("PAGER").unwrap_or_else(|_| {
                if cfg!(windows) {
                    "more".into()
                } else {
                    "less".into()
                }
            });
            let mut child = ProcessCommand::new(pager).stdin(Stdio::piped()).spawn()?;
            if let Some(stdin) = &mut child.stdin {
                stdin.write_all(text.as_bytes())?;
            }
            let _ = child.wait();
            Ok(())
        } else {
            print!("{}", text);
            Ok(())
        }
    }

    fn handle_command(cmdline: &str, memory: &[Scroll]) -> ChatMessage {
        let tokens: Vec<&str> = cmdline.trim_start_matches('/').split_whitespace().collect();
        let mut args = vec!["slash"];
        args.extend(tokens.iter());

        let mut app = Command::new("slash")
            .disable_help_subcommand(true)
            .subcommand(Command::new("help"))
            .subcommand(
                Command::new("scroll")
                    .subcommand(Command::new("list"))
                    .subcommand(Command::new("open").arg(arg!(<idx>))),
            );

        match app.clone().try_get_matches_from(args) {
            Ok(m) => match m.subcommand() {
                Some(("help", _)) | None => {
                    let txt =
                        "Available commands:\n  /help\n  /scroll list\n  /scroll open <idx>\n"
                            .to_string();
                    Self::system_msg(txt)
                }
                Some(("scroll", sub)) => match sub.subcommand() {
                    Some(("list", _)) => {
                        let mut out = String::new();
                        for (i, s) in memory.iter().enumerate() {
                            let lines = s.markdown_body.lines().count();
                            out.push_str(&format!(
                                "[{}] {} ({}, lines: {})\n",
                                i, s.title, s.scroll_type, lines
                            ));
                        }
                        Self::system_msg(out)
                    }
                    Some(("open", subm)) => {
                        let idx = subm.get_one::<String>("idx").unwrap();
                        match idx.parse::<usize>() {
                            Ok(i) if i < memory.len() => {
                                if let Err(e) = Self::pager_display(&memory[i].markdown_body) {
                                    return Self::system_msg(format!("{}", e));
                                }
                                Self::system_msg(String::new())
                            }
                            _ => Self::system_msg("Invalid index".into()),
                        }
                    }
                    _ => Self::system_msg("Unknown scroll command".into()),
                },
                Some((_, _)) => Self::system_msg("Unknown command".into()),
            },
            Err(e) => Self::system_msg(e.to_string()),
        }
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
        if user_input.trim_start().starts_with('/') {
            return Self::handle_command(user_input, memory);
        }

        let target_opt = ChatRouter::route_target(user_msg);
        let explicit = user_input.contains('@');

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

    /// Basic REPL loop that forwards user input to the target Construct.
    pub fn repl_loop(
        manager: &InvocationManager,
        aelren: &AelrenHerald,
        memory: &[Scroll],
        target: &str,
        stream: bool,
    ) {
        let mut session = ChatSession::new(Some(target.to_string()), None);
        let mut mood = EmotionalState::new(Vec::new(), 0.0, None);
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                let trimmed = line.trim();
                if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
                    break;
                }
                let _invocation = Invocation {
                    id: Uuid::new_v4(),
                    phrase: trimmed.to_string(),
                    invoker: "repl".into(),
                    invoked: target.to_string(),
                    tier: InvocationTier::True,
                    mode: InvocationMode::Read,
                    resonance_required: false,
                    timestamp: Utc::now(),
                };
                let reply =
                    Self::dispatch(&mut session, trimmed, manager, aelren, memory, &mut mood);
                if reply.role == "system" {
                    println!("{}", reply.content);
                } else {
                    println!("{} › {}", target, reply.content);
                }
                if !stream {
                    continue;
                }
            } else {
                break;
            }
        }
    }
}
