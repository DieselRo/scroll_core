use crate::chat::chat_dispatcher::ChatDispatcher;
use crate::chat::chat_session::ChatSession;
use crate::invocation::aelren::AelrenHerald;
use crate::invocation::invocation_manager::InvocationManager;
use crate::invocation::types::{Invocation, InvocationMode, InvocationTier};
use crate::trigger_loom::emotional_state::EmotionalState;
use crate::Scroll;
use anyhow::{anyhow, Result};
use chrono::Utc;
use ctrlc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use uuid::Uuid;

use crate::cli::theme::Theme;
use crate::sessions::database::{ensure_ready_with_url, get_db_connection};
use crate::sessions::database_session_service::DatabaseSessionService;
use crate::sessions::session_service::SessionService;
use crate::trigger_loom::config::{SymbolicRhythm, TriggerLoopConfig};
use crate::trigger_loom::engine::TriggerLoopEngine;
use console::Style;
use home::home_dir;
use rustyline::{error::ReadlineError, DefaultEditor};
use tokio::runtime::Runtime;

#[allow(clippy::too_many_arguments)]
pub fn run_chat(
    manager: &InvocationManager,
    aelren: &AelrenHerald,
    memory: &[Scroll],
    target: &str,
    _stream: bool,
    theme: Theme,
    show_banner: bool,
) -> Result<()> {
    let rt = Runtime::new()?;
    // Ensure DB connection is initialized (idempotent). Tests invoke the binary without prior init.
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://scroll_core.db".into());
    rt.block_on(async {
        let _ = ensure_ready_with_url(&db_url).await;
    });
    let conn = get_db_connection().clone();
    let session_svc = DatabaseSessionService::new(conn);
    if show_banner && std::env::var("SCROLL_CI").is_err() {
        let purple_bold = Style::new().color256(129).bold();
        println!("{}", purple_bold.apply_to("🔮 Scroll Core v0.2"));
    }

    let mut session = ChatSession::new(Some(target.to_string()), None);
    let mut mood = EmotionalState::new(Vec::new(), 0.0, None);
    let created = rt
        .block_on(session_svc.create_session("cli", "user", None, None))
        .map_err(|e| anyhow!(e.to_string()))?;
    let _session_id = created.id.clone();
    let mut created_session = created;

    let running = Arc::new(AtomicBool::new(true));
    let rflag = running.clone();
    ctrlc::set_handler(move || {
        println!("\nShutting down…");
        rflag.store(false, Ordering::SeqCst);
    })?;

    let mut rl = DefaultEditor::new()?;
    let hist_path = home_dir().map(|p| p.join(".scroll_core_history"));
    if let Some(path) = &hist_path {
        let _ = rl.load_history(path);
    }

    // Start a minimal background trigger loop (disabled in CI)
    if std::env::var("SCROLL_CI").is_err() {
        std::thread::spawn(move || {
            let config = TriggerLoopConfig {
                rhythm: SymbolicRhythm::EmotionDriven,
                max_invocations_per_tick: 1,
                allow_test_ticks: false,
                emotional_signature: None,
            };
            let mut engine = TriggerLoopEngine::new(config);
            // No NamedConstructs wired here yet; placeholder for Phase 6
            let mut none: Vec<Box<dyn crate::invocation::named_construct::NamedConstruct>> = vec![];
            // Run a few ticks and exit to avoid runaway thread in CLI sessions
            for _ in 0..3 {
                engine.tick_once(&mut none);
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        });
    }

    let prompt_user = theme.prompt_user.apply_to("You › ").to_string();
    while running.load(Ordering::SeqCst) {
        let readline = rl.readline(&prompt_user);
        let line = match readline {
            Ok(l) => l,
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(e) => return Err(e.into()),
        };
        let trimmed = line.trim();
        if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
            break;
        }
        let _ = rl.add_history_entry(trimmed);

        // Append user event (minimal content)
        let evt = crate::events::scroll_event::ScrollEvent::new(
            "user".to_string(),
            None,
            None,
            false,
            true,
            false,
            None,
        );
        let _ = rt
            .block_on(session_svc.append_event(&mut created_session, evt))
            .map_err(|e| anyhow!(e.to_string()));
        let _invocation = Invocation {
            id: Uuid::new_v4(),
            phrase: trimmed.to_string(),
            invoker: "cli".into(),
            invoked: target.to_string(),
            tier: InvocationTier::True,
            mode: InvocationMode::Read,
            resonance_required: false,
            timestamp: Utc::now(),
        };
        let reply =
            ChatDispatcher::dispatch(&mut session, trimmed, manager, aelren, memory, &mut mood);
        if reply.role == "system" {
            println!("{}", reply.content);
            let evt = crate::events::scroll_event::ScrollEvent::new(
                "system".to_string(),
                Some(crate::models::base_model::LLMResponseContent {
                    text: reply.content.clone(),
                }),
                None,
                false,
                true,
                false,
                None,
            );
            let _ = rt
                .block_on(session_svc.append_event(&mut created_session, evt))
                .map_err(|e| anyhow!(e.to_string()));
            continue;
        }
        println!("{} › {}", target, reply.content);
        let evt = crate::events::scroll_event::ScrollEvent::new(
            target.to_string(),
            Some(crate::models::base_model::LLMResponseContent {
                text: reply.content.clone(),
            }),
            None,
            false,
            true,
            false,
            None,
        );
        let _ = rt
            .block_on(session_svc.append_event(&mut created_session, evt))
            .map_err(|e| anyhow!(e.to_string()));
    }
    // Example ritual commands to persist and seal could be added here or in a dedicated CLI module
    // e.g., write current scroll to archive and optionally add to index.
    if let Some(path) = &hist_path {
        let _ = rl.save_history(path);
        if let Ok(contents) = std::fs::read_to_string(path) {
            let lines: Vec<&str> = contents.lines().rev().take(500).collect();
            let trimmed: String = lines.into_iter().rev().collect::<Vec<_>>().join("\n") + "\n";
            let _ = std::fs::write(path, trimmed);
        }
    }
    Ok(())
}
