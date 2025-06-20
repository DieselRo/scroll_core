// cache_manager.rs – Spiral Gate of Archive Memory

use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::archive::mythic_heat::MythicHeat;
use crate::archive::scroll_access_log::ScrollAccess;
use crate::core::cost_manager::InvocationCost;
use crate::schema::EmotionSignature;
use crate::scroll::Scroll;

/// Manages a memory-limited cache of scrolls based on mythic heat.
pub struct CacheManager {
    pub max_size: usize,
    pub active_scrolls: HashMap<Uuid, Scroll>,
    pub heat_scores: HashMap<Uuid, MythicHeat>,
}

impl CacheManager {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            active_scrolls: HashMap::new(),
            heat_scores: HashMap::new(),
        }
    }

    /// Attempts to add or update a scroll in cache.
    pub fn cache_scroll(
        &mut self,
        scroll: Scroll,
        emotion: &EmotionSignature,
        access: &ScrollAccess,
        cost: &InvocationCost,
    ) {
        let total_pressure =
            cost.cost_profile.token_pressure * 1.2 + cost.cost_profile.system_pressure * 0.8;

        let heat = MythicHeat::compute(scroll.id, emotion, access, total_pressure);
        self.heat_scores.insert(scroll.id, heat);
        self.active_scrolls.insert(scroll.id, scroll);
        self.prune_if_needed();
    }

    /// Ensures cache does not exceed max size by evicting coldest scrolls.
    fn prune_if_needed(&mut self) {
        if self.active_scrolls.len() <= self.max_size {
            return;
        }

        let mut scored: Vec<_> = self
            .heat_scores
            .iter()
            .map(|(id, heat)| (*id, heat.score()))
            .collect();
        scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        while self.active_scrolls.len() > self.max_size {
            if let Some((cold_id, _)) = scored.first() {
                self.active_scrolls.remove(cold_id);
                self.heat_scores.remove(cold_id);
                scored.remove(0);
            }
        }
    }

    pub fn get(&self, id: &Uuid) -> Option<&Scroll> {
        self.active_scrolls.get(id)
    }

    pub fn count(&self) -> usize {
        self.active_scrolls.len()
    }

    pub fn ids(&self) -> HashSet<Uuid> {
        self.active_scrolls.keys().copied().collect()
    }
}
