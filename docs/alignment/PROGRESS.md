---
title: Progress Log
status: active
audience: dev
---

# Progress Log

- Initialized docs portal and runtime sections
- Added doc indexing/classification CLI: `doc index`, `doc recent`, `doc classify`, `doc normalize`
- Auto-fix headers: `--fix-headers` option to insert minimal YAML
- Loader: recursive scan, skip `Deactivated scrolls/`, resilient to missing files
- Parser: lenient YAML handling; tolerant `scroll_type`, `status`, and string `emotion_signature`
- DB: initialize and run migrations on startup; CLI chat path also ensures migrations

# Progress Log

## 2025-08-08
- Initialized alignment docs.
- Phase 0 started:
  - Fixed stream flag logic in `scroll_core/src/main.rs`.
  - Built semantic index at startup (TokenEmbedder) in main entry paths.
  - Increased bus reply wait to 500ms in `chat_dispatcher` explicit agent path.
  - Removed stray root migration `m20250422_001344_add_session_state_column.rs`.
  - Documentation plan created: `docs/alignment/PLAN.md`, `DECISIONS.md`, `CHECKLIST.md`, `PROGRESS.md`.
  - Replaced Chat DB (sqlx) with SeaORM-backed `DatabaseSessionService` in `cli/chat.rs`.
  - Initialized SeaORM connection in `main.rs` using `DATABASE_URL`.
  - Added `archive/index.rs` module (read/write `scroll_index.yaml`).
  - Loader now prefers `scroll_index.yaml` (fallback to directory scan).
  - Build verified green.

Next: Phase 1 — add CLI index ops; complete `scroll_writer` round-trips; expand `validator` (structure/access) and expose ritual CLI.

Started: implement CLI index commands, scroll writer round-trip, validator v1.
  - Implemented Index CLI (list/add/remove).
  - Added validator v1: structure/access checks and write-deny for sealed.
 - Implemented Ritual CLI: validate/validate-all/write/seal using `ScrollWriter` and `validator`.
- `scroll_writer` round-trip completed (write/update/seal). 
- `write` ritual can auto-update `scroll_index.yaml` with `--update-index`.

Phase 2 (complete)
- Added cost gating in `InvocationManager` using `CostManager` decisions.
- Introduced SeaORM-backed invocation ledger and async logging (runtime path).
- Adapter to bridge `NamedConstruct` to `ConstructAI` for migration.

Phase 3 (in progress)
- Added Trigger Loop engine tick integration (CLI spawns short-lived loop when not in CI) using EmotionDriven rhythm.
- Routing policy in Aelren upgraded (tag hint, tone-based, mythscribe/first fallback).
- Ambient trigger helper added; tests cover intensity/tag thresholds.
