# Trigger Loom v1

The Trigger Loom is a periodic activation loop that modulates its rhythm by emotional state, wakes pulse-sensitive constructs within a per-tick budget, and logs all decisions to the ledger.

## Config Basics

- Rhythm: `SymbolicRhythm::{Constant|Dawn|Dusk|Spiral|EmotionDriven}`
- Budget: `max_invocations_per_tick`
- Emotion modulation: `EmotionDriven` uses the current `EmotionSignature` to map intensity to tick frequency.

## CLI Flags

- `--trigger-loop`: start the loop explicitly
- `--trigger-loop-ci`: deterministic CI mode (fixed rhythm, stable order)
- `--trigger-loop-budget <n>`: max allowed invocations per tick
- `--trigger-loop-period-ms <n>`: fixed tick period (overrides rhythm)

Environment: set `SC_TRIGGER_LOOP=1` to enable without flags.

## Ledger

Two SeaORM tables capture observability:

- `trigger_ticks`: id, tick_no, started_at, emotions_json, budget_in, budget_out
- `trigger_decisions`: id, tick_id, construct, decision_kind, est_cost_tokens, budget_remaining

Both allow and skip decisions are recorded with typed reasons: `Allow`, `Reject`, `Throttle`, `Cooldown`, `NotPulseSensitive`, `BudgetExceeded`.

## Emotion

`EmotionalState` now carries:

- `mood_trace`, `intensity`, optional `sigil_hint`
- `trigger_patterns: Vec<String>` and `sentiment: f32`
- `decay(per_sec)`: recency decay helper (e.g., 0.01/sec)

## Deterministic CI

Use `--trigger-loop-ci` to fix loop timing and ordering. Combine with `--trigger-loop-period-ms` for precise cadence.

