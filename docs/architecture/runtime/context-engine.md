---
title: Context Frame Engine
status: active
audience: dev
---

# Context Frame Engine

Builds a layered context for each invocation:

- Inputs: session transcript, archive excerpts, construct metadata
- Process: retrieve → filter → compose frame → trim tokens
- Output: prompt-ready structured context

Code
- `scroll_core/src/core/context_frame_engine.rs`
- Retrieval utilities under `archive/` and `memory/`

## Semantic Index Cache

To speed up warm starts, the semantic index persists to an OS-specific cache directory and supports incremental rebuilds:

- Location: defaults to the OS cache dir
  - Unix: `$XDG_CACHE_HOME/scrollcore/semantic_index/` or `~/.cache/scrollcore/semantic_index/`
  - Windows: `%LOCALAPPDATA%/ScrollCore/cache/semantic_index/`
- Files:
  - `semantic_index.v1.meta.json` – version and embedder metadata
  - `semantic_index.v1.vec.bin` – binary-encoded vectors (per-scroll records)
  - `fingerprints.v1.json` – per-scroll fingerprints keyed by path (mtime, size, content hash)
- Invalidation rules:
  - Content hash change of a scroll
  - Embedder model or embedding dimension change
  - Meta version change
- Behavior:
  - On startup, the loader attempts to read cache; if missing or corrupted, it falls back to a partial/full rebuild
  - Only invalidated entries are re-embedded; valid ones are reused from cache
  - Cache writes are atomic (write `*.tmp` and rename)
- Concurrency:
  - A lightweight advisory lock file prevents concurrent cache writers from clobbering files; additional writers skip cache writes

Environment toggles are documented in `docs/reference/config.md`.

## Thresholds and Reasons

Context selection limits can be tuned globally or per construct in `models.yaml`:

```yaml
context:
  decisions_verbose: false
  defaults:
    max_context_tokens: 3000
    min_relevance_score: 0.35
    recency_half_life_hours: 48
    max_items: 12
  constructs:
    Mythscribe:
      max_context_tokens: 4000
```

Environment variables override YAML values (`SC_CONTEXT_MAX_TOKENS`,
`SC_CONTEXT_Mythscribe_MAX_ITEMS`, etc.). `SC_CONTEXT_DECISIONS_VERBOSE=true`
enables detailed candidate logging to the context ledger.

Each candidate evaluated by the engine is tagged with a reason code:

| Code            | Meaning                                      |
|-----------------|----------------------------------------------|
| `Included`      | Candidate was added to the frame             |
| `SelfExclusion` | Triggering scroll filtered from candidates   |
| `LowRelevance`  | Below `min_relevance_score`                  |
| `MaxItems`      | Exceeds `max_items` limit                    |
| `TokenBudget`   | Would exceed `max_context_tokens`            |
| `IncludedFallback` | Explicitly seeded fallback candidate     |

Reason strings are stable and stored in the context ledger for later audit.


