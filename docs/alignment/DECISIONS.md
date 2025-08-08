---
title: Decisions (ADRs)
status: active
audience: dev
---

# Decisions (ADRs)

This is an index. Add links to ADRs as they are authored.

- ADR-0001: Placeholder – Runtime structure

# Architecture Decisions (ADR)

- ADR-0001: Sessions use SeaORM exclusively (2025-08-08)
  - Rationale: avoid schema drift; leverage migrations crate
  - Consequence: deprecate `cli/chat_db.rs` (sqlx) and migrate callers
  - Status: Implemented. CLI uses `DatabaseSessionService`; `DATABASE_URL` required.

- ADR-0002: Construct API standardizes on `ConstructAI` (2025-08-08)
  - Rationale: unify invocation manager integration
  - Consequence: provide adapter for any `NamedConstruct` remnants during transition
  - Status: Implemented. Adapter `invocation::adapters::NamedToAIAdapter<T>` added.

- ADR-0003: Invocation ledger persisted via SeaORM (2025-08-08)
  - Rationale: durable audit trail for invocations/costs
  - Consequence: new migration `invocation_ledger`; async logging after invoke
  - Status: Implemented.

- ADR-0004: Aelren routing policy includes tone-based fallback (2025-08-08)
  - Rationale: align symbolic routing with mood signal; ensure graceful default
  - Consequence: `AelrenHerald::suggest_construct` consults tags → tone → mythscribe/first
  - Status: Implemented.

- ADR-0005: Trigger Loom integrates short-lived background loop (2025-08-08)
  - Rationale: scaffold for emotion-driven ambient behavior without long-running threads in CLI/CI
  - Consequence: CLI spawns a few ticks when not in CI; future wiring to bus/constructs
  - Status: Implemented (scaffold).
