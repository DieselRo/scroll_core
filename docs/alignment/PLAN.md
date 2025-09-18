---
title: Roadmap Plan
status: active
audience: user, dev
---

# Roadmap Plan

- Milestone: Solidify runtime docs and CLI rituals
- Milestone: Configurable models and costs

## Master Plan (Temporary Guide)

Core Purpose
- Scroll Core lets named AI Constructs read/write/manage scrolls (YAML + Markdown) for lore and system instructions.
- Local-first ChatGPT-like runtime with persistent, structured knowledge.

Foundational Goals
- Local AI Runtime: invoke constructs; no manual file juggling; constructs update scrolls directly.
- Invocation System: symbolic and direct invocation; ContextFrameEngine routing.
- Construct Hierarchy: named constructs with domain-specific sub-constructs; archetypal expert roles.
- Archive Structure: YAML front matter + Markdown content; types for canon/protocol/reference; lifecycle stages.
- Database Integration: SeaORM + SQLite for sessions, events, indexing; informed by ADK lifecycles.
- CLI Features: invoke, symbolic, snapshot, list scrolls; `cli_orchestrator.rs` routes.

Phased Development Roadmap
1. Phase 1 — Active
   - Core Rust CLI runtime; read/write scrolls
   - Wire invocation + DB-backed sessions/events
2. Phase 2 — Coming
   - Named Construct chat window
   - Autonomous scroll creation + lore expansion
   - Full symbolic invocation integration
   - Begin autonomous writing milestone
3. Phase 3 — Long-Term
   - Distributed communal AI over shared archives
   - MMO/metaverse integration; NPC/worldbuilder systems

Design Philosophies
- Mythic metaphor as architecture; extensibility first; structured+fluid; recursive refinement via councils

Immediate Front-Loaded Work (to accelerate dev)
- Developer UX: improve CLI (`doc-index`, `doc classify`, `ritual` bundles) and snapshot flows
- Indexing: header-based scroll detection; dedupe/linkage; recency prioritization
- Models: registry abstraction; config profiles; cost gates
- Session DB: unify events; minimal metrics; opt-in tracing format
- LumenMind (frontend): basic chat client with construct selector and context preview

After Phase 8 (future path)
- Autonomous agenda and task graph per construct
- Cross-archive federation and conflict resolution
- Rich artifact system (content-addressed, typed, retained)

References
- Recent docs: see `docs/reference/doc-recent.md`
- Inventory/dupes: see `docs/reference/doc-inventory-summary.md`, `doc-dedupe.md`
- Normalizers: `cargo run -- doc --action classify --fix-headers` and `cargo run -- doc --action normalize`

# Alignment Plan – The Archive Runtime

This document captures the decisive plan to bring the runtime in line with the scrollbooks.

## Phase 0 — Stabilize (Day 1–2)
- Fix CLI stream flag logic. ✅
- Build semantic index at startup. ✅
- Bus usage cleanup in chat (set reasonable timeout). ✅
- Choose SeaORM for sessions; migrate away from raw sqlx in CLI. ✅

Acceptance: chat stable, semantic queries live, single DB direction chosen. ✅

## Phase 1 — Archive First-Class (Week 1)
- Implement `archive/index.rs` for `scroll_index.yaml` and admin CLI. (module added; integrate with loader next)
- Complete `scroll_writer` round-trips; serialization matches headers.
- Validator v1: structure + access checks.
- CLI rituals for seal/write/index/validate.

## Phase 2 — Invocation Engine (Week 2)
- Unify construct API on `ConstructAI` (adapter for `NamedConstruct` if needed).
- Pre-validate before invoke; minimal routing policy (tone/sigil/tier + fallback).
- Ledger with durable storage (SeaORM); record context decisions.
- Cost thresholds gate invoke.

## Phase 3 — Trigger Loom (Week 3)
- Activate tick loop; EmotionDriven rhythm; conversation-driven state.
- Ambient triggers using tags/emotion thresholds.

## Phase 4 — Dynamic Mirrors (Week 4)
- `open_threads` API + CLI rituals; contradiction scanner for duplicate titles/path collisions.
- CLI shows summaries post-validate/write.

## Phase 5 — Context Engine Upgrades (Week 4–5)
- Broad/Echo modes tuned; thresholds configurable; decisions logged.

## Phase 6 — Orchestrator Bus Patterns (Week 5)
- Orchestrated construct demo; dispatcher receive loop displays replies.

## Phase 7 — Sessions & Migrations (Week 5–6)
- SeaORM-only sessions; migrations consolidated in `migration/`.

## Phase 8 — Glyphskin-Ready CLI (Week 6+)
- Ritual-named commands; tone-driven styling; overlay-like status.

## Engineering Hygiene
- Tests: parser/validator/writer/context/cost/routing + E2E chat.
- Telemetry under feature flag.
- Docs: mapping from code to scrollbooks.
