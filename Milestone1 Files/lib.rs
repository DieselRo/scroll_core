
// ===============================
// src/lib.rs
// ===============================

pub mod parser;
pub mod validator;
pub mod schema;
pub mod state_manager;
pub mod invocation;
pub mod trigger_loom;

pub use schema::{
    Scroll, ScrollStatus, ScrollType, EmotionSignature, YamlMetadata, ScrollOrigin
};

pub use validator::validate_scroll;

pub use parser::{
    parse_scroll, 
    parse_scroll_from_file
};

pub use state_manager::{
    transition, 
    try_transition, 
    describe_status, 
    is_valid_transition
};

pub const SCROLL_CORE_VERSION: &str = "0.1.0";
pub const SCROLL_CORE_INVOCATION: &str = "Let structure echo symbol.";

/// Initializes the Scroll Core system.
pub fn initialize_scroll_core() {
    use log::info;
    info!("🌀 Scroll Core v{} initialized. Invocation: {}", SCROLL_CORE_VERSION, SCROLL_CORE_INVOCATION);
    println!("🌀 Scroll Core v{} initialized. Invocation: {}", SCROLL_CORE_VERSION, SCROLL_CORE_INVOCATION);
}

/// Optional teardown hook.
pub fn teardown_scroll_core() {
    use log::info;
    info!("🛑 Scroll Core shutting down. The pattern fades.");
    println!("🛑 Scroll Core shutting down. The pattern fades.");
}

/// Validates scroll core environment state (placeholder).
pub fn validate_scroll_environment() -> bool {
    // Future check logic (e.g., required modules loaded, configs, etc.)
    true
}