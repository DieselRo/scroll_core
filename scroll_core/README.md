# Scroll Core – Implementation Notes

This crate implements the runtime for The Archive. For the vision/spec, see the `scrolls/` directory. For ongoing alignment work, see `docs/alignment/` at the repo root.

Key entry points:
- `src/main.rs`: CLI entry
- `archive/`: loading, in-memory archive, semantic index
- `core/`: context framing, cost manager
- `invocation/`: constructs and invocation manager
- `chat/`: interactive chat and routing

## Migration Notes (Ledger Service)

Invocation logging now uses a long-lived async service rather than per-call
threads. Events are sent over a bounded channel and written once the database is
ready. Under load, excess events are dropped predictably. To verify the ledger:

1. Set `DATABASE_URL` (defaults to `sqlite://scroll_core.db?mode=rwc`).
2. Run the CLI or chat; invocations will accumulate in the `invocation_ledger`
   table.
3. To roll back, omit starting the service and ledger writes become no-ops.
