//! Trait and in-memory implementation for storing scrolls during runtime.
//! This component provides basic indexing and heat tracking for loaded scrolls.
//! See [Archive Memory](../../AGENTS.md#scrollwriter) for overview.
// archive_memory.rs – Archive Memory Trait and Initial Implementation
//=======================================================================

use std::collections::HashMap;
use uuid::Uuid;

use crate::archive::error::ArchiveError;
use crate::archive::semantic_index::{Embedder, SemanticIndex};
use crate::schema::EmotionSignature;
use crate::scroll::Scroll;
use log::info;

/// Defines access methods for interacting with the Archive's scroll memory.
pub trait ArchiveMemory {
    fn get_all_scrolls(&self) -> Vec<&Scroll>;
    fn get_scroll_by_id(&self, id: Uuid) -> Option<&Scroll>;
    fn get_scrolls_by_tag(&self, tag: &str) -> Vec<&Scroll>;
    fn count(&self) -> usize;
    fn query_semantic(&self, input: &str, k: usize) -> Vec<(Scroll, f32)>;
}

/// Simple Phase 1 implementation that holds all scrolls in memory.
pub struct InMemoryArchive {
    scrolls: HashMap<Uuid, Scroll>,
    semantic_index: Option<SemanticIndex>,
}

impl InMemoryArchive {
    pub fn new(scrolls: Vec<Scroll>) -> Self {
        let scrolls_map = scrolls.into_iter().map(|s| (s.id, s)).collect();
        Self {
            scrolls: scrolls_map,
            semantic_index: None,
        }
    }
}

impl InMemoryArchive {
    /// Match any of the provided tags
    pub fn query_by_tags(&self, tags: &[String]) -> Vec<Scroll> {
        self.scrolls
            .values()
            .filter(|scroll| scroll.yaml_metadata.tags.iter().any(|t| tags.contains(t)))
            .cloned()
            .collect()
    }

    /// Match scrolls based on emotional similarity
    pub fn query_by_emotion(&self, target: EmotionSignature) -> Vec<Scroll> {
        self.scrolls
            .values()
            .filter(|scroll| {
                let sig = &scroll.yaml_metadata.emotion_signature;
                sig.tone == target.tone
                    && (sig.intensity.unwrap_or(0.0) - target.intensity.unwrap_or(0.0)).abs() < 0.3
            })
            .cloned()
            .collect()
    }

    /// Match scrolls that symbolically link to the given scroll
    pub fn query_by_links(&self, id: &Uuid) -> Vec<Scroll> {
        self.scrolls
            .values()
            .filter(|scroll| scroll.linked_scrolls.iter().any(|link| &link.target == id))
            .cloned()
            .collect()
    }

    pub fn semantic_index_len(&self) -> usize {
        self.semantic_index
            .as_ref()
            .map(|i| i.vectors.len())
            .unwrap_or(0)
    }

    /// Build semantic vector index for all scrolls.
    pub fn build_semantic_index(&mut self, embedder: &dyn Embedder) -> Result<(), ArchiveError> {
        let scrolls: Vec<Scroll> = self.scrolls.values().cloned().collect();
        let index = SemanticIndex::build(&scrolls, embedder)?;
        self.semantic_index = Some(index);
        Ok(())
    }

    /// Query scrolls using semantic similarity of title and tags.
    pub fn query_semantic(&self, input: &str, k: usize) -> Vec<(Scroll, f32)> {
        if let Some(idx) = &self.semantic_index {
            idx.query(input, k)
                .into_iter()
                .filter_map(|(id, score)| self.scrolls.get(&id).cloned().map(|s| (s, score)))
                .collect()
        } else {
            info!("Semantic index not built; returning empty results");
            Vec::new()
        }
    }
}

impl ArchiveMemory for InMemoryArchive {
    fn get_all_scrolls(&self) -> Vec<&Scroll> {
        self.scrolls.values().collect()
    }

    fn get_scroll_by_id(&self, id: Uuid) -> Option<&Scroll> {
        self.scrolls.get(&id)
    }

    fn get_scrolls_by_tag(&self, tag: &str) -> Vec<&Scroll> {
        self.scrolls
            .values()
            .filter(|scroll| scroll.yaml_metadata.tags.iter().any(|t| t == tag))
            .collect()
    }

    fn count(&self) -> usize {
        self.scrolls.len()
    }

    fn query_semantic(&self, input: &str, k: usize) -> Vec<(Scroll, f32)> {
        InMemoryArchive::query_semantic(self, input, k)
    }
}

// Future implementation placeholder for cache-aware archive model.
// pub struct HybridCacheArchive { /* future phase */ }
// impl ArchiveMemory for HybridCacheArchive { /* ... */ }
