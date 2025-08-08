---
title: Invocation Engine
status: active
audience: dev
system: invocation
---

# Invocation Engine

At a glance
- Entry points: `AelrenHerald`, `InvocationManager`
- Responsibilities: framing, routing, cost gating, ledger logging
- Related: ADR-0003 (ledger), ADR-0004 (routing)

## Components
- AelrenHerald: builds context, suggests construct (tag → tone → fallback)
- InvocationManager: cost assessment (CostManager), invoke via `ConstructRegistry`, async DB ledger (`invocation_ledger`)

## Code
- `scroll_core/src/invocation/aelren.rs`
- `scroll_core/src/invocation/invocation_manager.rs`
- `scroll_core/src/invocation/ledger.rs`

## CLI
- MVP chat: `cargo run -- chat mythscribe --no-stream`


