use crate::chat::chat_dispatcher::ChatDispatcher;
use crate::chat::chat_session::ChatSession;
use crate::invocation::aelren::AelrenHerald;
use crate::invocation::invocation_manager::InvocationManager;
use crate::trigger_loom::emotional_state::EmotionalState;
use crate::Scroll;
use anyhow::Result;
use rustyline::{error::ReadlineError, DefaultEditor};

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
    let mut rl = DefaultEditor::new()?;
    let mut session = ChatSession::new(Some(target.to_string()), None);
    let mut mood = EmotionalState::new(Vec::new(), 0.0, None);
    let session_id = rt.block_on(db.create_session())?;

    loop {
        let line = match rl.readline("You › ") {
            Ok(l) => l,
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(e) => return Err(e.into()),
        };
        let trimmed = line.trim();
        if trimmed.eq_ignore_ascii_case("exit") || trimmed == "/exit" {
            break;
        }
        let _ = rl.add_history_entry(trimmed);
        rt.block_on(db.log_event(&session_id, "user", trimmed))?;
        let reply =
            ChatDispatcher::dispatch(&mut session, trimmed, manager, aelren, memory, &mut mood);
        println!("{} › {}", target, reply.content);
        rt.block_on(db.log_event(&session_id, target, &reply.content))?;
    }
    Ok(())
}
