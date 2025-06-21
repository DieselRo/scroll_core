use crate::chat::chat_dispatcher::ChatDispatcher;
use crate::chat::chat_session::ChatSession;
use crate::invocation::aelren::AelrenHerald;
use crate::invocation::invocation_manager::InvocationManager;
use crate::invocation::types::{Invocation, InvocationMode, InvocationTier};
use crate::trigger_loom::emotional_state::EmotionalState;
use crate::Scroll;
use anyhow::Result;
use chrono::Utc;
use ctrlc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use uuid::Uuid;

use crate::cli::chat_db::ChatDb;
use crate::cli::theme::Theme;
use ansi_term::Colour;
use home::home_dir;
use rustyline::{error::ReadlineError, DefaultEditor};
use tokio::runtime::Runtime;

pub fn run_chat(
    manager: &InvocationManager,
    aelren: &AelrenHerald,
    memory: &[Scroll],
    target: &str,
    _stream: bool,
    db: &ChatDb,
    theme: Theme,
    show_banner: bool,
) -> Result<()> {
    let rt = Runtime::new()?;
    if show_banner && std::env::var("SCROLL_CI").is_err() {
        println!("{}", Colour::Purple.bold().paint("🔮 Scroll Core v0.2"));
    }

    let mut session = ChatSession::new(Some(target.to_string()), None);
    let mut mood = EmotionalState::new(Vec::new(), 0.0, None);
    let session_id = rt.block_on(db.create_session())?;

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

    let prompt_user = theme.prompt_user.paint("You › ").to_string();
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

        if let Err(e) = rt.block_on(db.log_event(&session_id, "user", trimmed)) {
            eprintln!(
                "Failed to log event for session '{}', role 'user': {e}",
                session_id
            );
        }
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
        println!("{} › {}", target, reply.content);
        if let Err(e) = rt.block_on(db.log_event(&session_id, target, &reply.content)) {
            eprintln!(
                "Failed to log event for session '{}', target '{}': {e}",
                session_id, target
            );
        }
    }
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
