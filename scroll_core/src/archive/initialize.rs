use std::fs;
use std::path::Path;

use crate::archive::archive_loader::load_scrolls_from_directory;
use crate::archive::error::ArchiveError;
use crate::archive::scroll_access_log::ScrollAccess;
use crate::cache_manager::CacheManager;
use crate::core::cost_manager::{
    ContextCost, CostDecision, CostProfile, InvocationCost, SystemCost,
};
use crate::scroll::Scroll;

/// Ensures the archive directory exists, creating it and a `.gitkeep` file if
/// needed.
pub fn ensure_archive_dir(path: &Path) -> Result<(), ArchiveError> {
    if !path.exists() {
        fs::create_dir_all(path).map_err(ArchiveError::Io)?;
        let keep = path.join(".gitkeep");
        fs::write(keep, "").map_err(ArchiveError::Io)?;
    }
    Ok(())
}

/// Loads the archive from the given path and seeds a cache with the scrolls.
/// The cache size is set to match the number of loaded scrolls.
pub fn load_with_cache<P: AsRef<Path>>(path: P) -> Result<(Vec<Scroll>, CacheManager), String> {
    let scrolls = load_scrolls_from_directory(path)?;
    let mut cache = CacheManager::new(scrolls.len());

    for scroll in &scrolls {
        let access = ScrollAccess::new();
        let cost = InvocationCost {
            context: ContextCost {
                token_estimate: 0,
                context_span: 0,
                relevance_score: 0.0,
            },
            system: SystemCost {
                cpu_cycles: 0.0,
                memory_used_mb: 0.0,
                io_ops: 0,
                scrolls_touched: 0,
            },
            decision: CostDecision::Allow,
            cost_profile: CostProfile {
                system_pressure: 0.0,
                token_pressure: 0.0,
                symbolic_origin: None,
            },
            rejection_origin: None,
            hesitation_signal: None,
            poetic_rejection: None,
            symbolic_echo: None,
            emotion_tension: None,
        };
        cache.cache_scroll(scroll.clone(), &scroll.emotion_signature, &access, &cost);
    }

    Ok((scrolls, cache))
}
