//! src/main.rs – Scroll Core entry point
//! Run normally:  `cargo run`
//! Demo mode:     `cargo run -- --demo examples/multi_agent.yaml`

#![warn(unused_imports)]

use anyhow::Result;
use std::path::Path;
use std::str::FromStr;

use clap::{Parser, Subcommand};
use dotenvy::dotenv;
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

    /// Start the Trigger Loom loop (manual start)
    #[arg(long = "trigger-loop", action = clap::ArgAction::SetTrue, default_value_t = false)]
    trigger_loop: bool,
    /// Deterministic CI mode for Trigger Loom
    #[arg(long = "trigger-loop-ci", action = clap::ArgAction::SetTrue, default_value_t = false)]
    trigger_loop_ci: bool,
    /// Per-tick budget (max invocations)
    #[arg(long = "trigger-loop-budget")]
    trigger_loop_budget: Option<usize>,
    /// Tick period in milliseconds
    #[arg(long = "trigger-loop-period-ms")]
    trigger_loop_period_ms: Option<u64>,
    /// Profile for trigger loop: demo or ci
    #[arg(long = "profile", value_parser = clap::builder::PossibleValuesParser::new(["demo", "ci"]))]
    profile: Option<String>,
    /// Explain context decisions during trigger-loop constructs
    #[arg(long = "explain-context", action = clap::ArgAction::SetTrue, default_value_t = false)]
    explain_context: bool,
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
        /// Explain context selection decisions
        #[arg(long = "explain-context", action = clap::ArgAction::SetTrue, default_value_t = false)]
        explain_context: bool,
    },
    /// Inspect context decision ledger
    Context {
        /// Number of recent frames to show
        #[arg(long, default_value_t = 10)]
        limit: usize,
        /// Also print candidate detail rows
        #[arg(long = "details", action = clap::ArgAction::SetTrue, default_value_t = false)]
        details: bool,
        #[arg(
            long = "export",
            value_parser = clap::builder::PossibleValuesParser::new(["json", "yaml"])
        )]
        export: Option<String>,
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
        action: Option<String>,
        /// Positional action for compatibility: `ritual validate --file ...`
        #[arg(index = 1)]
        action_positional: Option<String>,
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
    /// Manage persistent open threads
    #[command(name = "open-threads")]
    OpenThreads {
        #[arg(long, value_parser = clap::builder::PossibleValuesParser::new(["create", "list", "close", "reopen", "nudge"]))]
        action: String,
        /// Thread title (create)
        #[arg(long = "title")]
        title: Option<String>,
        /// Scroll path the thread refers to (create/list filter)
        #[arg(long = "scroll")]
        scroll: Option<String>,
        /// Optional assignee label (create)
        #[arg(long = "assignee")]
        assignee: Option<String>,
        /// Optional priority (create) LOW|MEDIUM|HIGH
        #[arg(long = "priority")]
        priority: Option<String>,
        /// Optional comma-separated tags (create)
        #[arg(long = "tags")]
        tags: Option<String>,
        /// Optional ISO8601 due date (create)
        #[arg(long = "due-at")]
        due_at: Option<String>,
        /// Filter by status for list (OPEN|IN_PROGRESS|BLOCKED|CLOSED)
        #[arg(long = "status")]
        status: Option<String>,
        /// Filter by current user assignment
        #[arg(long = "mine", action = clap::ArgAction::SetTrue, default_value_t = false)]
        mine: bool,
        /// Filter overdue (due_at < now and not CLOSED)
        #[arg(long = "overdue", action = clap::ArgAction::SetTrue, default_value_t = false)]
        overdue: bool,
        /// Filter by priority in list
        #[arg(long = "filter-priority")]
        list_priority: Option<String>,
        /// Filter by tags in list (comma separated)
        #[arg(long = "filter-tags")]
        list_tags: Option<String>,
        /// Sort key: created|updated|priority
        #[arg(long = "sort")]
        sort: Option<String>,
        /// Limit rows for list
        #[arg(long = "limit")]
        limit: Option<u64>,
        /// Close by id (close)
        id: Option<String>,
        /// Optional reason on close
        #[arg(long = "reason")]
        reason: Option<String>,
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
        explain_context,
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
            let _ = scroll_core::sessions::database::ensure_ready_with_url(&db_url).await;
        });
        let mut archive = InMemoryArchive::new(scrolls.clone());
        // Build semantic index for context modes that rely on it
        {
            let embedder = TokenEmbedder;
            let _ = archive.build_semantic_index(&embedder);
        }
        let thresholds = model_registry.context_for(construct);
        let engine = ContextFrameEngine::new(&archive, ContextMode::Narrow)
            .with_thresholds(thresholds)
            .with_construct_label(construct);

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
        let mut aelren = AelrenHerald::new(engine, vec![construct.clone()]);
        aelren.explain_context = *explain_context;
        aelren.decisions_verbose = model_registry.context_decisions_verbose();
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

    // Trigger Loom start (explicit via CLI or env)
    if cli.trigger_loop || std::env::var("SC_TRIGGER_LOOP").ok().as_deref() == Some("1") {
        use scroll_core::invocation::constructs::{
            mythscribe_gate::MythscribeGate, pulse_echo::PulseEcho, pulse_logger::PulseLogger,
        };
        use scroll_core::orchestra::Bus;
        use scroll_core::trigger_loom::config::TriggerLoopProfile;
        use scroll_core::orchestra::OrchestratedConstruct;

        // DB init + migrations (for decision ledger)
        let db_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite://scroll_core.db?mode=rwc".into());
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            let _ = scroll_core::sessions::database::ensure_ready_with_url(&db_url).await;
        });

        if cli.explain_context {
            std::env::set_var("SC_EXPLAIN_CONTEXT", "1");
        }

        // Start ledger service
        let (ledger_handle, ledger_service) =
            scroll_core::invocation::ledger_service::start(64, 256);

        // Load scrolls to check tags for ambient triggers
        let (scrolls, _cache) = initialize_scroll_core()?;
        let pulse_enabled = scrolls
            .iter()
            .any(|s| s.yaml_metadata.tags.iter().any(|t| t == "pulse"));

        // Determine profile
        let profile = if cli.trigger_loop_ci
            || cli.profile.as_deref() == Some("ci")
        {
            TriggerLoopProfile::Ci
        } else {
            TriggerLoopProfile::Demo
        };

        // Build base config from profile and override via CLI
        use scroll_core::trigger_loom::config::SymbolicRhythm;
        let mut cfg = profile.config();
        if let Some(b) = cli.trigger_loop_budget {
            cfg.max_invocations_per_tick = b;
        }
        if let Some(ms) = cli.trigger_loop_period_ms {
            let hz = (1000.0f32 / (ms as f32)).max(0.001);
            cfg.rhythm = SymbolicRhythm::Constant(hz);
        }

        let mut engine = scroll_core::trigger_loom::engine::TriggerLoopEngine::new(cfg.clone());
        if matches!(profile, TriggerLoopProfile::Ci) {
            engine = engine.with_deterministic_seed(42);
        }

        // Bus wiring and constructs
        let bus = Bus::new();
        let mut echo = PulseEcho::default().with_ledger(ledger_handle.clone());
        echo.enabled = pulse_enabled;
        echo.attach_bus(bus.clone());
        let mut logger = PulseLogger::default().with_ledger(ledger_handle.clone());
        logger.attach_bus(bus.clone());
        let mut gate = MythscribeGate::default().with_ledger(ledger_handle.clone());
        gate.ci_mode = matches!(profile, TriggerLoopProfile::Ci);
        gate.attach_bus(bus.clone());

        let mut constructs: Vec<
            Box<dyn scroll_core::invocation::named_construct::NamedConstruct>,
        > = vec![Box::new(echo), Box::new(gate)];

        println!("▶️ Starting Trigger Loom (press Ctrl-C to stop)...");
        let tick_limit = profile.tick_limit();
        if matches!(profile, TriggerLoopProfile::Ci) {
            engine.start_loop(&mut constructs, tick_limit);
        } else {
            use scroll_core::trigger_loom::emotional_state::EmotionalState;
            let mut state = EmotionalState::new(vec!["start".into()], 0.5, None);
            if pulse_enabled {
                state.trigger_patterns.push("pulse".into());
            }
            engine.start_loop_with_emotion(&mut constructs, state, tick_limit);
        }

        drop(constructs);
        drop(logger);
        drop(ledger_handle);
        // Graceful shutdown of ledger worker
        ledger_service.shutdown_blocking(std::time::Duration::from_millis(250));
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
        action_positional,
        file,
        update_index,
    }) = &cli.command
    {
        let archive_dir =
            std::env::var("SCROLL_CORE_ARCHIVE_DIR").unwrap_or_else(|_| "scrolls".into());
        let path = Path::new(&archive_dir);
        ensure_archive_dir(path)?;
        let action_eff = action_positional
            .as_ref()
            .or(action.as_ref())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "ritual action is required (validate | validate-all | write | seal)"
                )
            })?;
        match action_eff.as_str() {
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

    // Open Threads API + CLI
    if let Some(Commands::OpenThreads {
        action,
        title,
        scroll,
        assignee,
        status,
        mine,
        overdue,
        list_priority,
        list_tags,
        sort,
        limit,
        id,
        reason,
        priority,
        tags,
        due_at,
    }) = &cli.command
    {
        // Ensure DB is ready
        let db_url =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://scroll_core.db".into());
        let rt = tokio::runtime::Runtime::new()?;
        let conn = rt.block_on(async {
            scroll_core::sessions::database::ensure_ready_with_url(&db_url)
                .await
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        })?;
        use scroll_core::threads::thread_state_service::ThreadStateService;
        use scroll_core::threads::types::{Priority, ThreadStatus};
        match action.as_str() {
            "create" => {
                let t = title
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("--title is required for create"))?;
                let s = scroll
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("--scroll is required for create"))?;
                let prio = if let Some(p) = priority.as_ref() {
                    Priority::from_str(p).map_err(|e| anyhow::anyhow!(e))?
                } else {
                    Priority::Medium
                };
                let tags_vec: Option<Vec<String>> = tags.as_ref().map(|s| {
                    s.split(',')
                        .map(|t| t.trim().to_string())
                        .collect::<Vec<String>>()
                });
                let due = if let Some(d) = due_at.as_ref() {
                    Some(
                        chrono::DateTime::parse_from_rfc3339(d)
                            .map_err(|e| {
                                anyhow::anyhow!(format!("invalid --due-at (RFC3339): {}", e))
                            })?
                            .with_timezone(&chrono::Utc),
                    )
                } else {
                    None
                };
                let svc = ThreadStateService::new(&conn);
                let rec = tokio::runtime::Runtime::new()?.block_on(async {
                    svc.create(s, t, assignee.as_deref(), prio, tags_vec, due, None, "cli")
                        .await
                        .map_err(|e| anyhow::anyhow!(e.to_string()))
                })?;
                println!(
                    "created: {} | {} | {} | {} | prio={} | tags={}",
                    rec.id,
                    rec.status,
                    rec.scroll_path,
                    rec.title,
                    rec.priority,
                    rec.tags.unwrap_or_default()
                );
            }
            "list" => {
                let svc = ThreadStateService::new(&conn);
                // parse status if provided
                let s_parsed: Option<ThreadStatus> = if let Some(s) = status.as_ref() {
                    Some(ThreadStatus::from_str(s).map_err(|e| anyhow::anyhow!(e))?)
                } else {
                    None
                };
                let who = if *mine {
                    std::env::var("USER")
                        .ok()
                        .or_else(|| std::env::var("USERNAME").ok())
                } else {
                    None
                };
                let prio = if let Some(p) = list_priority.as_ref() {
                    Some(Priority::from_str(p).map_err(|e| anyhow::anyhow!(e))?)
                } else {
                    None
                };
                let tags_vec: Option<Vec<String>> = list_tags.as_ref().map(|s| {
                    s.split(',')
                        .map(|t| t.trim().to_ascii_lowercase())
                        .filter(|t| !t.is_empty())
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>()
                });
                let rows = tokio::runtime::Runtime::new()?.block_on(async {
                    svc.list(
                        s_parsed,
                        scroll.as_deref(),
                        *limit,
                        who.as_deref(),
                        prio,
                        tags_vec,
                        *overdue,
                        sort.as_deref(),
                    )
                    .await
                    .map_err(|e| anyhow::anyhow!(e.to_string()))
                })?;
                for r in rows {
                    println!(
                        "{} | {} | {} | {} | {} | prio={} | assignee={} | due_at={}",
                        r.id,
                        r.status,
                        r.scroll_path,
                        r.title,
                        r.created_at,
                        r.priority,
                        r.assignee.unwrap_or_default(),
                        r.due_at
                            .map(|d| d.to_rfc3339())
                            .unwrap_or_else(|| "".into())
                    );
                }
            }
            "nudge" => {
                // emit system-note nudges for blocked or overdue threads
                let svc = scroll_core::threads::thread_autocapture::ThreadAutocapture::new(&conn);
                let count = tokio::runtime::Runtime::new()?.block_on(async {
                    svc.nudge_blocked_or_overdue()
                        .await
                        .map_err(|e| anyhow::anyhow!(e.to_string()))
                })?;
                println!("nudged {} thread(s)", count);
            }
            "reopen" => {
                let id = id
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("<id> is required for reopen"))?;
                let svc = ThreadStateService::new(&conn);
                let _ = tokio::runtime::Runtime::new()?.block_on(async {
                    svc.update_status(id, ThreadStatus::Open, reason.as_deref(), "cli")
                        .await
                        .map_err(|e| anyhow::anyhow!(e.to_string()))
                })?;
                println!("reopened: {}", id);
            }
            "close" => {
                let id = id
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("<id> is required for close"))?;
                let svc = ThreadStateService::new(&conn);
                let _ = tokio::runtime::Runtime::new()?.block_on(async {
                    svc.update_status(
                        id,
                        scroll_core::threads::types::ThreadStatus::Closed,
                        reason.as_deref(),
                        "cli",
                    )
                    .await
                    .map_err(|e| anyhow::anyhow!(e.to_string()))
                })?;
                if true {
                    println!("closed: {}", id);
                } else {
                    println!("not-found: {}", id);
                }
            }
            _ => unreachable!(),
        }
        return Ok(());
    }

    if let Some(Commands::Context {
        limit,
        details,
        export,
    }) = &cli.command
    {
        // Ensure DB is ready
        let db_url =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://scroll_core.db".into());
        let rt = tokio::runtime::Runtime::new()?;
        let conn = rt.block_on(async {
            scroll_core::sessions::database::ensure_ready_with_url(&db_url)
                .await
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        })?;
        use scroll_core::invocation::context_ledger::{candidate, frame};
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
        use serde::Serialize;
        let frames: Vec<frame::Model> = tokio::runtime::Runtime::new()?.block_on(async move {
            frame::Entity::find()
                .order_by_desc(frame::Column::Timestamp)
                .limit(*limit as u64)
                .all(conn)
                .await
                .unwrap_or_default()
        });
        if let Some(fmt) = export {
            #[derive(Serialize)]
            struct FrameExport {
                frame: frame::Model,
                candidates: Vec<candidate::Model>,
            }
            let mut out = Vec::new();
            for f in frames {
                let mut candidates_vec = Vec::new();
                if *details {
                    let conn2 = scroll_core::sessions::database::get_db_connection().clone();
                    candidates_vec = tokio::runtime::Runtime::new()?.block_on(async move {
                        candidate::Entity::find()
                            .filter(candidate::Column::FrameId.eq(f.frame_id))
                            .order_by_asc(candidate::Column::Timestamp)
                            .all(&conn2)
                            .await
                            .unwrap_or_default()
                    });
                }
                out.push(FrameExport {
                    frame: f,
                    candidates: candidates_vec,
                });
            }
            let serialized = if fmt == "json" {
                serde_json::to_string_pretty(&out)?
            } else {
                serde_yaml::to_string(&out)?
            };
            println!("{}", serialized);
        } else {
            println!("Latest {} context frames:", frames.len());
            for f in &frames {
                println!(
                    "- frame={} construct={} max_tokens={} max_items={} min_rel={:.2} half_life_h={:.1} candidates={} included={} excluded={} build_ms={} at {}",
                    f.frame_id,
                    f.construct,
                    f.max_tokens,
                    f.max_items,
                    f.min_relevance,
                    f.half_life_hours,
                    f.total_candidates,
                    f.included_count,
                    f.excluded_count,
                    f.build_ms,
                    f.timestamp
                );
                if *details {
                    let conn2 = scroll_core::sessions::database::get_db_connection().clone();
                    let rows: Vec<candidate::Model> =
                        tokio::runtime::Runtime::new()?.block_on(async move {
                            candidate::Entity::find()
                                .filter(candidate::Column::FrameId.eq(f.frame_id))
                                .order_by_asc(candidate::Column::Timestamp)
                                .all(&conn2)
                                .await
                                .unwrap_or_default()
                        });
                    for r in rows {
                        let mark = if r.included { "✔" } else { "✖" };
                        println!(
                            "  {} {} score={:.2} age_h={:.1} tokens={}/{} reason={}",
                            mark,
                            r.candidate_path.unwrap_or_else(|| "<unknown>".into()),
                            r.score,
                            r.recency_hours,
                            r.running_tokens,
                            r.max_tokens,
                            r.reason
                        );
                    }
                }
            }
        }
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
            let thresholds = model_registry.context_for("Mythscribe");
            let engine = ContextFrameEngine::new(&archive, ContextMode::Narrow)
                .with_thresholds(thresholds)
                .with_construct_label("mythscribe");

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
                if let Ok(rt) = tokio::runtime::Runtime::new() {
                    rt.block_on(async {
                        let _ =
                            scroll_core::sessions::database::ensure_ready_with_url(&db_url).await;
                    });
                }
            }

            // Start ledger service after ensuring DB/migrations
            let (ledger_handle, ledger_service) =
                scroll_core::invocation::ledger_service::start(64, 256);

            let manager = InvocationManager::new(registry).with_ledger(ledger_handle.clone());
            let mut aelren = AelrenHerald::new(engine, vec!["mythscribe".into()]);
            if std::env::var("SC_EXPLAIN_CONTEXT").ok().as_deref() == Some("1") {
                aelren.explain_context = true;
            }

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
    let thresholds = ModelRegistry::get_global()
        .map(|r| r.context_for("Mythscribe"))
        .unwrap_or_default();
    let engine = ContextFrameEngine::new(&archive, ContextMode::Narrow)
        .with_thresholds(thresholds)
        .with_construct_label("mythscribe");
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
    let mut aelren = AelrenHerald::new(engine, vec!["mythscribe".into()]);
    if std::env::var("SC_EXPLAIN_CONTEXT").ok().as_deref() == Some("1") {
        aelren.explain_context = true;
    }

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
