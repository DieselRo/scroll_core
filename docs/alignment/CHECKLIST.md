# Alignment Checklist

## Phase 0
- [x] Fix stream flag logic
- [x] Build semantic index at startup
- [x] Bus timeout sane default
- [x] Decide DB layer (SeaORM)
- [x] Draft replacement for `cli/chat_db.rs`

## Phase 1
- [x] Implement `archive/index.rs`
- [x] Loader uses `scroll_index.yaml` (fallback to scan)
- [x] `scroll_writer` round-trip (write/update/seal wired to CLI)
- [x] Validator v1 (structure/access)
- [x] Ritual CLI (seal/write/validate/validate-all; write supports --update-index)

## Phase 2
- [x] Unify construct API on `ConstructAI` (adapter available)
- [x] Pre-validate in `InvocationManager` (cost gating)
- [x] Routing policy stub (tone with mythscribe fallback)
- [x] Ledger (SeaORM) + async logging
- [x] Cost thresholds gate invoke

## Phase 3
- [x] Trigger loop v1 (emotion-modulated; budgeted; CLI-gated)
- [x] Emotion-driven rhythm activation with constructs
- [x] Ambient triggers (tags/emotion thresholds)

## Phase 4
- [ ] Open Threads API + CLI
- [ ] Contradiction detection stub

## Phase 5
- [ ] Context thresholds configurable; decisions logged

## Phase 6
- [ ] Orchestrated construct demo; dispatcher receive loop

## Phase 7
- [ ] SeaORM-only sessions; migrations green

## Phase 8
- [ ] Ritual-named TUI elements; tone-driven styling
