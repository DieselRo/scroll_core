//! src/main.rs – Scroll Core entry point
//! Run normally:  `cargo run`
//! Demo mode:     `cargo run -- --demo examples/multi_agent.yaml`

#![warn(unused_imports)]

use anyhow::Result;
use std::path::Path;

use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use scroll_core::chat::chat_dispatcher::ChatDispatcher;
use scroll_core::cli::{chat::run_chat, chat_db::ChatDb, theme::ThemeKind};
use scroll_core::{
    archive::archive_memory::InMemoryArchive,
    archive::initialize::ensure_archive_dir,
    core::{
        construct_registry::ConstructRegistry,
        context_frame_engine::{ContextFrameEngine, ContextMode},
    },
    initialize_scroll_core,
    invocation::{
        aelren::AelrenHerald,
        constructs::openai_construct::{Mythscribe, OpenAIClient},
        invocation_manager::InvocationManager,
    },
    parser::parse_scroll,
    teardown_scroll_core,
    trigger_loom::emotional_state::EmotionalState,
};

/// CLI flags recognised by Scroll Core.
#[derive(Parser)]
#[command(name = "scroll_core")]
struct Cli {
    /// Path to a demo scroll that should trigger a cooperative run
    #[arg(long)]
    demo: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start an interactive chat with a Construct
    Chat {
        construct: String,
        #[arg(
            long = "stream",
            action = clap::ArgAction::SetTrue,
            default_value_t = false,
            help = "Enable streaming output",
            conflicts_with = "no_stream"
        )]
        stream: bool,
        #[arg(long = "no-stream", action = clap::ArgAction::SetTrue, default_value_t = false)]
        no_stream: bool,
        #[arg(long, default_value = "dark")]
        theme: ThemeKind,
        #[arg(long = "no-banner", action = clap::ArgAction::SetTrue, default_value_t = false)]
        no_banner: bool,
    },
}

fn main() -> Result<()> {
    dotenv().ok();

    scroll_core::init_tracing()?;

    #[cfg(feature = "metrics")]
    scroll_core::telemetry::init();

    let cli = Cli::parse();

    if let Some(Commands::Chat {
        construct,
        stream,
        no_stream,
        theme,
        no_banner,
    }) = &cli.command
    {
        let archive_dir =
            std::env::var("SCROLL_CORE_ARCHIVE_DIR").unwrap_or_else(|_| "scrolls".into());
        ensure_archive_dir(Path::new(&archive_dir))?;
        let (scrolls, _cache) = initialize_scroll_core()?;
        let archive = InMemoryArchive::new(scrolls.clone());
        let engine = ContextFrameEngine::new(&archive, ContextMode::Narrow);

        let mut registry = ConstructRegistry::new();
        if std::env::var("SCROLL_CORE_USE_MOCK").is_ok() {
            registry.insert(
                "mythscribe",
                scroll_core::invocation::constructs::mockscribe::Mockscribe,
            );
        } else {
            let mythscribe = Mythscribe::new(
                OpenAIClient::new_from_env(),
                "You are Mythscribe, the poetic analyst of sacred scrolls.".into(),
            );
            registry.insert("mythscribe", mythscribe);
        }

        let manager = InvocationManager::new(registry);
        let aelren = AelrenHerald::new(engine, vec![construct.clone()]);
        let rt = tokio::runtime::Runtime::new()?;
        let db_path = std::env::var("CHAT_DB_PATH").unwrap_or_else(|_| "scroll_core.db".into());
        let db = rt.block_on(ChatDb::open(&db_path))?;
        let stream_enabled = *stream && *no_stream;
        let theme_struct = theme.styles();
        run_chat(
            &manager,
            &aelren,
            &scrolls,
            construct,
            stream_enabled,
            &db,
            theme_struct,
            !*no_banner,
        )?;
        teardown_scroll_core();
        return Ok(());
    }

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
    let archive = InMemoryArchive::new(scrolls.clone());
    let engine = ContextFrameEngine::new(&archive, ContextMode::Narrow);
    let mut reg = ConstructRegistry::new();
    let myth = Mythscribe::new(
        OpenAIClient::new_from_env(),
        "You are Mythscribe, the poetic analyst of sacred scrolls.".into(),
    );
    reg.insert("mythscribe", myth);
    let manager = InvocationManager::new(reg);

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
