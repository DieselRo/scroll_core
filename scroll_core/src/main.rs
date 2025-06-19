//! src/main.rs – Scroll Core entry point
//! Run normally:  `cargo run`
//! Demo mode:     `cargo run -- --demo examples/multi_agent.yaml`

use std::path::Path;
use anyhow::Result;

use clap::Parser;
use dotenvy::dotenv;
use scroll_core::chat::chat_dispatcher::ChatDispatcher;
use scroll_core::{
    archive::archive_memory::InMemoryArchive,
    core::{
        construct_registry::ConstructRegistry,
        context_frame_engine::{ContextFrameEngine, ContextMode},
    },
    invocation::{
        aelren::AelrenHerald,
        constructs::openai_construct::{Mythscribe, OpenAIClient},
        invocation_manager::InvocationManager,
    },
    trigger_loom::emotional_state::EmotionalState,
    initialize_scroll_core, parser::parse_scroll, teardown_scroll_core,
};

/// CLI flags recognised by Scroll Core.
#[derive(Parser)]
#[command(name = "scroll_core")]
struct Cli {
    /// Path to a demo scroll that should trigger a cooperative run
    #[arg(long)]
    demo: Option<String>,
}

fn main() -> Result<()> {
    dotenv().ok();

    #[cfg(feature = "metrics")]
    scroll_core::telemetry::init();

    let cli = Cli::parse();

    // ─── Demo path ──────────────────────────────────────────────────────────────
    if let Some(demo_path) = cli.demo {
        run_demo(&demo_path)?;
        teardown_scroll_core();
        return Ok(());
    }

    // ─── Normal start-up ───────────────────────────────────────────────────────
    match initialize_scroll_core() {
        Ok((scrolls, _cache)) => {
            println!("✨ Scroll Core is active. Awaiting construct cadence...\n");

            let archive = InMemoryArchive::new(scrolls.clone());
            let engine = ContextFrameEngine::new(&archive, ContextMode::Narrow);

            // Seed construct registry
            let mut registry = ConstructRegistry::new();
            let mythscribe = Mythscribe::new(
                OpenAIClient::new_from_env(),
                "You are Mythscribe, the poetic analyst of sacred scrolls.".into(),
            );
            registry.insert("mythscribe", mythscribe);

            let manager = InvocationManager::new(registry);
            let aelren = AelrenHerald::new(engine, vec!["mythscribe".into()]);

            scroll_core::system::cli_orchestrator::run_cli(&manager, &aelren, &scrolls);
        }
        Err(e) => eprintln!("❌ Initialization failed: {e}"),
    }

    teardown_scroll_core();
    Ok(())
}

// ───────────────────────────────────────────────────────────────────────────────
// Demo helper
// ───────────────────────────────────────────────────────────────────────────────
fn run_demo<P: AsRef<std::path::Path>>(path: P) -> Result<()> {
    use scroll_core::chat::chat_session::{ChatMessage, ChatSession};

    // 1️⃣  init core
    let (mut scrolls, _cache) = initialize_scroll_core()?;

    // 2️⃣  load demo scroll the same way
    let raw = std::fs::read_to_string(path)?;
    let demo_scroll = parse_scroll(&raw)?;
    scrolls.push(demo_scroll.clone());

    // 3️⃣  tiny runtime
    let archive  = InMemoryArchive::new(scrolls.clone());
    let engine   = ContextFrameEngine::new(&archive, ContextMode::Narrow);
    let mut reg  = ConstructRegistry::new();
    let myth     = Mythscribe::new(
        OpenAIClient::new_from_env(),
        "You are Mythscribe, the poetic analyst of sacred scrolls.".into(),
    );
    reg.insert("mythscribe", myth);
    let manager  = InvocationManager::new(reg);

    let mut session = ChatSession::new(None, None);
    let mut mood = EmotionalState::new(Vec::new(), 0.0, None);
    let _dispatcher = ChatDispatcher::new(&manager, &engine);
    let aelren = AelrenHerald::new(engine, vec!["mythscribe".into()]);

    let user_msg = "@validator Please inspect The Ballad";
    let reply: ChatMessage = ChatDispatcher::dispatch(
        &mut session,
        user_msg,
        &manager,
        &aelren,
        &scrolls,
        &mut mood,
    );

    println!("\n=== Assistant replied ===\n{}\n", reply.content);
    Ok(())
}
