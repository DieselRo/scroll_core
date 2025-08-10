---
title: Configuration Reference
status: active
audience: user, dev
---

# Configuration

Environment variables (loaded via `dotenvy` if `.env` is present):

- OPENAI_API_KEY
  - Required for OpenAI-backed constructs
  - Example: `OPENAI_API_KEY=sk-...`

- DATABASE_URL
  - Database connection string
  - Defaults: `sqlite://scroll_core.db?mode=rwc` (main) or `sqlite://scroll_core.db` (CLI)
  - Example: `DATABASE_URL=sqlite://scroll_core.db?mode=rwc`
  - Notes: SeaORM is the sole ORM (sessions + ledger). Legacy SQLx paths have been removed. No OpenSSL is required; HTTP clients use rustls.

- SCROLL_CORE_ARCHIVE_DIR
  - Directory containing scrolls archive
  - Default: `scrolls`

- SCROLL_CORE_USE_MOCK
  - If set, registers mock constructs for deterministic testing

- SCROLL_CI
  - If set, disables banner and background trigger loop in CLI

- SCROLL_TRACE_FORMAT
  - Tracing output format; when set to `json`, emits structured logs

- PAGER
  - Pager command for rich CLI output; defaults to `less -R` or `more` on Windows

Notes
- `.env` is loaded at startup; tests may override via process env
- See `scroll_core/src/tracing.rs` for tracing behavior

Model & Cost Configuration
- See `docs/reference/models-config.md` for provider/model selection and cost thresholds.


## Semantic Index Cache

Environment variables controlling the semantic index cache:

- `SC_DISABLE_INDEX_CACHE`
  - If set to `1`, disables loading/saving the semantic index cache.
- `SC_INDEX_CACHE_DIR`
  - Override cache directory (default is OS-specific cache path).
- `SC_REBUILD_INDEX`
  - If set to `1`, forces a full rebuild of the semantic index (ignores any cache).
- `SC_REINDEX_PATH`
  - If set to a specific scroll file path, forces a re-embed of that single entry.
- `SC_EMBEDDER_MODEL`
  - Override embedder model string recorded in the cache meta (changes here invalidate cache).
- `SC_EMBEDDING_DIM`
  - Override embedding dimension recorded in the cache meta (changes here invalidate cache).

CLI flags:
- `--rebuild-index` – set `SC_REBUILD_INDEX=1` for this run.
- `--reindex <path>` – set `SC_REINDEX_PATH` to target a single scroll for reindex.



