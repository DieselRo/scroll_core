use crate::chat::chat_dispatcher::ChatDispatcher;
use crate::chat::chat_session::ChatSession;
use crate::invocation::aelren::AelrenHerald;
use crate::invocation::invocation_manager::InvocationManager;
use crate::trigger_loom::emotional_state::EmotionalState;
use crate::invocation::types::{Invocation, InvocationMode, InvocationTier};
use chrono::Utc;
use uuid::Uuid;
use crate::Scroll;
use anyhow::Result;
use std::io::{self, BufRead};
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use ctrlc;

use crate::cli::chat_db::ChatDb;
use tokio::runtime::Runtime;

pub fn run_chat(
    manager: &InvocationManager,
    aelren: &AelrenHerald,
    memory: &[Scroll],
    target: &str,
    _stream: bool,
    db: &ChatDb,
) -> Result<()> {
    let rt = Runtime::new()?;
    let mut session = ChatSession::new(Some(target.to_string()), None);
    let mut mood = EmotionalState::new(Vec::new(), 0.0, None);
    let session_id = rt.block_on(db.create_session())?;

    let running = Arc::new(AtomicBool::new(true));
    let rflag = running.clone();
    ctrlc::set_handler(move || {
        println!("\nShutting down…");
        rflag.store(false, Ordering::SeqCst);
    })?;

    let stdin = io::stdin();
    while running.load(Ordering::SeqCst) {
        let mut line = String::new();
        if stdin.lock().read_line(&mut line)? == 0 {
            break;
        }
        let trimmed = line.trim();
        if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
            break;
        }

        rt.block_on(db.log_event(&session_id, "user", trimmed))?;
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
        let reply = ChatDispatcher::dispatch(&mut session, trimmed, manager, aelren, memory, &mut mood);
        println!("{} › {}", target, reply.content);
        rt.block_on(db.log_event(&session_id, target, &reply.content))?;
    }
    Ok(())
}
