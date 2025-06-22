//! Error types for archive operations such as loading and embedding.
//! See [Archive Memory](../../AGENTS.md#scrollwriter) for how these are surfaced.
// src/archive/error.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArchiveError {
    #[error("no scrolls were provided")]
    EmptyScrollSet,
    #[error("embedding model not found")]
    MissingModel,
    #[error("embedding failed: {0}")]
    EmbeddingFailure(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
