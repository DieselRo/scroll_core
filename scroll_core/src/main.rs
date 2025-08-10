//! src/main.rs – Scroll Core entry point
//! Run normally:  `cargo run`
//! Demo mode:     `cargo run -- --demo examples/multi_agent.yaml`

#![warn(unused_imports)]

use anyhow::Result;
use std::path::Path;

use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use migration::MigratorTrait;
use scroll_core::chat::chat_dispatcher::ChatDispatcher;
use scroll_core::cli::{chat::run_chat, theme::ThemeKind};
use scroll_core::models::model_registry::ModelRegistry;
use scroll_core::{
    archive::archive_memory::InMemoryArchive,
    archive::initialize::ensure_archive_dir,
    archive::semantic_index::TokenEmbedder,
    core::{
        construct_registry::ConstructRegistry,
        context_frame_engine::{ContextFrameEngine, ContextMode},
    },
    initialize_scroll_core,
    invocation::{
        aelren::AelrenHerald, constructs::openai_construct::Mythscribe,
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

    /// Force full rebuild of semantic index cache
    #[arg(long = "rebuild-index", action = clap::ArgAction::SetTrue, default_value_t = false)]
    rebuild_index: bool,

    /// Force reindex of a single scroll path
    #[arg(long = "reindex")]
    reindex: Option<String>,

    /// Print resolved model config and exit
    #[arg(long = "print-model-config", action = clap::ArgAction::SetTrue, default_value_t = false)]
    print_model_config: bool,

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
    /// Archive index operations
    Index {
        #[arg(long, value_parser = clap::builder::PossibleValuesParser::new(["list", "add", "remove"]))]
        action: String,
        #[arg(long)]
        file: Option<String>,
    },
    /// Ritual operations over scrolls (validate, write, seal)
    Ritual {
        #[arg(long, value_parser = clap::builder::PossibleValuesParser::new(["validate", "validate-all", "write", "seal"]))]
        action: String,
        #[arg(long)]
        file: Option<String>,
        /// Also add to scroll_index.yaml when writing
        #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
        update_index: bool,
    },
    /// Document utilities (index, classify, recent)
    Doc {
        #[arg(long, value_parser = clap::builder::PossibleValuesParser::new(["index", "classify", "recent", "normalize", "master-plan"]))]
        action: String,
        /// Apply minimal headers to scrolls/ files missing valid YAML (DANGEROUS)
        #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
        fix_headers: bool,
    },
}

fn main() -> Result<()> {
    dotenv().ok();

    scroll_core::init_tracing()?;

    #[cfg(feature = "metrics")]
    scroll_core::telemetry::init();

    let cli = Cli::parse();

    // Build registry once; it maintains env-only behavior when no YAML is present
    let model_registry = ModelRegistry::from_env_and_file(None)
        .map_err(|e| anyhow::anyhow!(format!("model registry error: {e}")))?;
    let model_registry = std::sync::Arc::new(model_registry);
    let _ = ModelRegistry::set_global(model_registry.clone());

    if cli.print_model_config {
        let redacted = model_registry.effective_config();
        let yaml =
            serde_yaml::to_string(&redacted).unwrap_or_else(|_| "<failed to serialize>".into());
        println!("{}", yaml);
        return Ok(());
    }

    if let Some(Commands::Chat {
        construct,
        stream,
        no_stream,
        theme,
        no_banner,
    }) = &cli.command
    {
        if cli.rebuild_index {
            std::env::set_var("SC_REBUILD_INDEX", "1");
        }
        if let Some(p) = &cli.reindex {
            std::env::set_var("SC_REINDEX_PATH", p);
        }
        let archive_dir =
            std::env::var("SCROLL_CORE_ARCHIVE_DIR").unwrap_or_else(|_| "scrolls".into());
        ensure_archive_dir(Path::new(&archive_dir))?;
        let (scrolls, _cache) = initialize_scroll_core()?;
        // Initialize database connection for session logging
        let db_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite://scroll_core.db?mode=rwc".into());
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            if scroll_core::sessions::database::init_sqlite_connection(&db_url)
                .await
                .is_ok()
            {
                let _ = migration::Migrator::up(
                    scroll_core::sessions::database::get_db_connection(),
                    None,
                )
                .await;
            }
        });
        let mut archive = InMemoryArchive::new(scrolls.clone());
        // Build semantic index for context modes that rely on it
        {
            let embedder = TokenEmbedder;
            let _ = archive.build_semantic_index(&embedder);
        }
        let engine = ContextFrameEngine::new(&archive, ContextMode::Narrow);

        let mut registry = ConstructRegistry::new();
        if std::env::var("SCROLL_CORE_USE_MOCK").is_ok() {
            registry.insert(
                "mythscribe",
                scroll_core::invocation::constructs::mockscribe::Mockscribe,
            );
        } else {
            let spec = model_registry
                .clone()
                .by_construct("Mythscribe")
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            let client = scroll_core::invocation::llm::factory::from_spec(&spec)
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            let mythscribe = Mythscribe::new(
                client,
                "You are Mythscribe, the poetic analyst of sacred scrolls.".into(),
            );
            registry.insert("mythscribe", mythscribe);
        }
        // Optional: attach a pulse-sensitive construct to bus later (Phase 6)

        // Start ledger service after DB init/migrations
        let (ledger_handle, ledger_service) =
            scroll_core::invocation::ledger_service::start(64, 256);

        let manager = InvocationManager::new(registry).with_ledger(ledger_handle.clone());
        let aelren = AelrenHerald::new(engine, vec![construct.clone()]);
        let stream_enabled = *stream && !*no_stream;
        let theme_struct = theme.styles();
        run_chat(
            &manager,
            &aelren,
            &scrolls,
            construct,
            stream_enabled,
            theme_struct,
            !*no_banner,
        )?;
        // Graceful shutdown of ledger worker
        ledger_service.shutdown_blocking(std::time::Duration::from_millis(250));
        teardown_scroll_core();
        return Ok(());
    }

    if let Some(Commands::Index { action, file }) = &cli.command {
        let archive_dir =
            std::env::var("SCROLL_CORE_ARCHIVE_DIR").unwrap_or_else(|_| "scrolls".into());
        let path = Path::new(&archive_dir);
        ensure_archive_dir(path)?;
        match action.as_str() {
            "list" => {
                scroll_core::cli::index::index_list(path).map_err(anyhow::Error::msg)?;
            }
            "add" => {
                let f = file
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("--file is required for add"))?;
                scroll_core::cli::index::index_add(path, &f).map_err(anyhow::Error::msg)?;
                println!("Added {} to index", f);
            }
            "remove" => {
                let f = file
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("--file is required for remove"))?;
                scroll_core::cli::index::index_remove(path, &f).map_err(anyhow::Error::msg)?;
                println!("Removed {} from index", f);
            }
            _ => unreachable!(),
        }
        return Ok(());
    }

    if let Some(Commands::Ritual {
        action,
        file,
        update_index,
    }) = &cli.command
    {
        let archive_dir =
            std::env::var("SCROLL_CORE_ARCHIVE_DIR").unwrap_or_else(|_| "scrolls".into());
        let path = Path::new(&archive_dir);
        ensure_archive_dir(path)?;
        match action.as_str() {
            "validate" => {
                let f = file
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("--file is required for validate"))?;
                scroll_core::cli::ritual::ritual_validate(path, &f)?;
            }
            "validate-all" => {
                scroll_core::cli::ritual::ritual_validate_all(path)?;
            }
            "write" => {
                let f = file
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("--file is required for write"))?;
                scroll_core::cli::ritual::ritual_write(path, &f, *update_index)?;
            }
            "seal" => {
                let f = file
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("--file is required for seal"))?;
                scroll_core::cli::ritual::ritual_seal(path, &f)?;
            }
            _ => unreachable!(),
        }
        return Ok(());
    }

    if let Some(Commands::Doc {
        action,
        fix_headers,
    }) = &cli.command
    {
        match action.as_str() {
            "index" => scroll_core::cli::docs::doc_index()?,
            "classify" => scroll_core::cli::docs::doc_classify()?,
            "recent" => scroll_core::cli::docs::doc_recent()?,
            "normalize" => scroll_core::cli::docs::doc_normalize_headers()?,
            "master-plan" => scroll_core::cli::docs::doc_generate_master_plan()?,
            _ => unreachable!(),
        }
        if *fix_headers {
            scroll_core::cli::docs::doc_fix_headers()?;
            println!("Applied minimal headers to scrolls/ candidates.");
        }
        println!("Docs: {} generated under docs/reference", action);
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

            if cli.rebuild_index {
                std::env::set_var("SC_REBUILD_INDEX", "1");
            }
            if let Some(p) = &cli.reindex {
                std::env::set_var("SC_REINDEX_PATH", p);
            }
            let mut archive = InMemoryArchive::new(scrolls.clone());
            {
                let embedder = TokenEmbedder;
                let _ = archive.build_semantic_index(&embedder);
            }
            let engine = ContextFrameEngine::new(&archive, ContextMode::Narrow);

            // Seed construct registry
            let mut registry = ConstructRegistry::new();
            let spec = model_registry
                .clone()
                .by_construct("Mythscribe")
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            let client = scroll_core::invocation::llm::factory::from_spec(&spec)
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            let mythscribe = Mythscribe::new(
                client,
                "You are Mythscribe, the poetic analyst of sacred scrolls.".into(),
            );
            registry.insert("mythscribe", mythscribe);

            // Ensure DB connection and migrations for CLI ledger/session logging
            {
                let db_url = std::env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "sqlite://scroll_core.db?mode=rwc".into());
                if !scroll_core::sessions::database::is_initialized() {
                    if let Ok(rt) = tokio::runtime::Runtime::new() {
                        rt.block_on(async {
                            if scroll_core::sessions::database::init_sqlite_connection(&db_url)
                                .await
                                .is_ok()
                            {
                                let _ = migration::Migrator::up(
                                    scroll_core::sessions::database::get_db_connection(),
                                    None,
                                )
                                .await;
                            }
                        });
                    }
                }
            }

            // Start ledger service after ensuring DB/migrations
            let (ledger_handle, ledger_service) =
                scroll_core::invocation::ledger_service::start(64, 256);

            let manager = InvocationManager::new(registry).with_ledger(ledger_handle.clone());
            let aelren = AelrenHerald::new(engine, vec!["mythscribe".into()]);

            scroll_core::system::cli_orchestrator::run_cli(&manager, &aelren, &scrolls);

            // shutdown ledger
            ledger_service.shutdown_blocking(std::time::Duration::from_millis(250));
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
    let mut archive = InMemoryArchive::new(scrolls.clone());
    {
        let embedder = TokenEmbedder;
        let _ = archive.build_semantic_index(&embedder);
    }
    let engine = ContextFrameEngine::new(&archive, ContextMode::Narrow);
    let mut reg = ConstructRegistry::new();
    let spec = ModelRegistry::get_global()
        .unwrap()
        .by_construct("Mythscribe")
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let client = scroll_core::invocation::llm::factory::from_spec(&spec)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let myth = Mythscribe::new(
        client,
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
