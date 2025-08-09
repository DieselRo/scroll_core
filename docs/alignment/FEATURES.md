# Scroll Core – Implemented Features (Phase 0–3)

This document enumerates the main features currently implemented and working in the codebase, reflecting the state of development up through Phase 3 in-progress.

## Archive and Scroll System
- Archive loader prioritizes `scrolls/scroll_index.yaml`, falling back to directory scan.
- `scroll_index.yaml` management via CLI: `index --action list|add|remove`.
- `ScrollWriter` round-trip: write, update, seal; CLI rituals for write/seal.
- Validator v1 ensures required YAML fields, non-empty tags, and denies writes to sealed scrolls.

## CLI Rituals
- `ritual --action validate --file <path>`: validates one scroll.
- `ritual --action validate-all`: validates all markdown scrolls in archive.
- `ritual --action write --file <path> [--update-index]`: persists scroll and optionally updates index.
- `ritual --action seal --file <path>`: sets sealed status and persists.

## Invocation Engine
- Central `InvocationManager` with cost assessment and gating (pre-validate).
- `ConstructRegistry` for registering and invoking `ConstructAI` constructs.
- Aelren framing (`AelrenHerald`) for building context and routing (tag/tone/fallback); actual DB ledger logging is performed post-invoke.
- Adapter to bridge `NamedConstruct` to `ConstructAI` for transition.

## Cost and Ledger
- `CostManager` computes pressures and a decision (`Allow`/`Reject`/`Throttle`).
- Invocations are DB-logged to `invocation_ledger` (SeaORM) asynchronously after execution.

## Sessions
- Unified on SeaORM; global DB connection initializer available.
- Session service implemented for appending events and closing sessions.

## Semantic Index
- `InMemoryArchive` builds semantic index at startup; used by context engine.

## Orchestrator Bus
- `OrchestratedConstruct` support in registry; bus cloning and attach logic.

## ADK Integration (Stub)
- `adk_example` binary stub to keep build green without ADK.

---

Phase 3 items
- Trigger Loop scaffold integrated into CLI with EmotionDriven rhythm (short-lived ticks).
- Ambient trigger helper to gate actions by tags and emotion thresholds.
- Next: add pulse-sensitive construct and ambient activations to bus/ledger.


