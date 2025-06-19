// ===============================
// src/lib.rs
// ===============================

pub mod adk; // Agent Development Kit integration
pub mod archive;
pub mod artifact;
pub mod artifacts;
pub mod cache_manager;
pub mod chat;
pub mod construct_ai;
pub mod constructs;
pub mod core;
pub mod database;
pub mod events;
pub mod invocation;
pub mod memory;
pub mod models;
pub mod orchestra;
pub mod parser;
pub mod runner;
pub mod schema;
pub mod scroll;
pub mod scroll_writer;
pub mod sessions;
pub mod state_manager;
pub mod system;
#[cfg(feature = "metrics")]
pub mod telemetry;
pub mod tools;
pub mod trigger_loom;
pub mod validator;

pub use cache_manager::CacheManager;
pub use schema::{EmotionSignature, ScrollStatus, ScrollType, YamlMetadata};
pub use scroll::{Scroll, ScrollOrigin};
pub use validator::validate_scroll;

pub use parser::{parse_scroll, parse_scroll_from_file};

pub use state_manager::{describe_status, is_valid_transition, transition, try_transition};

pub const SCROLL_CORE_VERSION: &str = "0.1.0";
pub const SCROLL_CORE_INVOCATION: &str = "Let structure echo symbol.";

/// Initializes the Scroll Core system and loads the scroll archive.

pub fn initialize_scroll_core() -> Result<(Vec<Scroll>, CacheManager), String> {
    use crate::archive::initialize::load_with_cache;
    use log::info;
    use std::path::Path;

    let archive_path = Path::new("scrolls/");

    info!("🌀 Scroll Core v{} initializing...", SCROLL_CORE_VERSION);
    println!("🌀 Scroll Core v{} initializing...", SCROLL_CORE_VERSION);

    let (scrolls, cache) = load_with_cache(archive_path)?;

    info!("✅ Loaded {} scroll(s).", scrolls.len());
    println!("✅ Loaded {} scroll(s).", scrolls.len());

    Ok((scrolls, cache))
}
/// Optional teardown hook.
pub fn teardown_scroll_core() {
    use log::info;
    info!("🛑 Scroll Core shutting down. The pattern fades.");
    println!("🛑 Scroll Core shutting down. The pattern fades.");
}

/// Validates scroll core environment state (placeholder).
pub fn validate_scroll_environment() -> bool {
    use std::env;
    use std::fs;

    dotenvy::dotenv().ok();

    if env::var("OPENAI_API_KEY").is_err() {
        return false;
    }

    match fs::read_dir("scrolls/") {
        Ok(mut entries) => entries.next().is_some(),
        Err(_) => false,
    }
}
