use scroll_core::archive::archive_memory::InMemoryArchive;
use scroll_core::core::construct_registry::ConstructRegistry;
use scroll_core::core::context_frame_engine::{ContextFrameEngine, ContextMode};
use scroll_core::invocation::aelren::AelrenHerald;
use scroll_core::invocation::constructs::openai_construct::{Mythscribe, OpenAIClient};
use scroll_core::invocation::invocation_manager::InvocationManager;
use scroll_core::system::cli_orchestrator::run_cli;
use scroll_core::{initialize_scroll_core, teardown_scroll_core};

use dotenvy::dotenv;

fn main() {
    dotenv().ok();
    #[cfg(feature = "metrics")]
    scroll_core::telemetry::init();
    println!("🔑 OPENAI key: {:?}", std::env::var("OPENAI_API_KEY"));

    match initialize_scroll_core() {
        Ok((scrolls, _cache)) => {
            println!("✨ Scroll Core is active. Awaiting construct cadence...\n");

            let archive = InMemoryArchive::new(scrolls.clone());
            let engine = ContextFrameEngine::new(&archive, ContextMode::Narrow);

            // ✅ Build and seed registry
            let mut registry = ConstructRegistry::new();

            let mythscribe = Mythscribe::new(
                OpenAIClient::new_from_env(),
                "You are Mythscribe, the poetic analyst of sacred scrolls.".into(),
            );

            registry.insert("mythscribe", mythscribe);

            let manager = InvocationManager::new(registry);
            let aelren = AelrenHerald::new(engine, vec!["mythscribe".into()]);

            run_cli(&manager, &aelren, &scrolls);
        }
        Err(e) => {
            eprintln!("❌ Initialization failed: {}", e);
        }
    }

    teardown_scroll_core();
}
