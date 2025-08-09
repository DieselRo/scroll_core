# Scroll Core – Implementation Notes

This crate implements the runtime for The Archive. For the vision/spec, see the `scrolls/` directory. For ongoing alignment work, see `docs/alignment/` at the repo root.

Key entry points:
- `src/main.rs`: CLI entry
- `archive/`: loading, in-memory archive, semantic index
- `core/`: context framing, cost manager
- `invocation/`: constructs and invocation manager
- `chat/`: interactive chat and routing


## Migration Notes (Ledger Service)

Invocation logging now uses a long-lived asynchronous service. Each
invocation sends a `LedgerEvent` over a bounded channel; the worker
buffers events until the database connection is ready and then flushes
them. This removes per-invocation thread spawns and prevents panics when
the database has not been initialised.


