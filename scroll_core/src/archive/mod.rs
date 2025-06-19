//! Persistent scroll archive utilities.
//!
//! This module exposes functions to load scrolls from disk and track
//! archival metrics. Typical usage:
//!
//! ```rust,no_run
//! use scroll_core::archive::initialize::load_with_cache;
//! use std::path::Path;
//!
//! let (scrolls, cache) = load_with_cache(Path::new("scrolls/"))?;
//! println!("Loaded {} scrolls", scrolls.len());
//! # Ok::<(), String>(())
//! ```

pub mod archive_loader;
pub mod archive_memory;
pub mod error;
pub mod initialize;
pub mod mythic_heat;
pub mod scroll_access_log;
pub mod semantic_index;
