// ===============================
// src/lib.rs
// ===============================
#![warn(unused_imports)]

pub mod archive;
pub mod artifact;
pub mod artifacts;
pub mod cache_manager;
pub mod chat;
pub mod cli;
pub mod construct_ai;
pub mod constructs;
pub mod core;
pub mod database;
pub mod errors;
pub mod events;
pub mod invocation;
pub mod memory;
pub mod metrics;
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
pub mod tracing;
pub mod trigger_loom;
pub mod validator;

pub use cache_manager::CacheManager;
pub use errors::MetricError;
pub use metrics::clamp_finite;
pub use schema::{EmotionSignature, ScrollStatus, ScrollType, YamlMetadata};
pub use scroll::{Scroll, ScrollOrigin};
pub use validator::validate_scroll;

pub use parser::{parse_scroll, parse_scroll_from_file};
pub use tracing::{init_tracing, init_tracing_for_test};

use anyhow::Result;

pub use state_manager::{describe_status, is_valid_transition, transition, try_transition};

pub const SCROLL_CORE_VERSION: &str = "0.2.0";
pub const SCROLL_CORE_INVOCATION: &str = "Let structure echo symbol.";

/// Initializes the Scroll Core system and loads the scroll archive.
pub fn initialize_scroll_core() -> Result<(Vec<Scroll>, CacheManager)> {
    use crate::archive::initialize::load_with_cache;
    use log::info;
    use std::path::Path;

    let archive_dir = std::env::var("SCROLL_CORE_ARCHIVE_DIR").unwrap_or_else(|_| "scrolls".into());
    let archive_path = Path::new(&archive_dir);

    info!("🌀 Scroll Core v{} initializing...", SCROLL_CORE_VERSION);
    println!("🌀 Scroll Core v{} initializing...", SCROLL_CORE_VERSION);

    let (scrolls, cache) = load_with_cache(archive_path).map_err(anyhow::Error::msg)?;

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

    let archive_dir = env::var("SCROLL_CORE_ARCHIVE_DIR").unwrap_or_else(|_| "scrolls".into());
    match fs::read_dir(&archive_dir) {
        Ok(mut entries) => entries.next().is_some(),
        Err(_) => false,
    }
}
