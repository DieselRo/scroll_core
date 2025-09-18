# Scroll Core – Implementation Notes

This crate implements the runtime for The Archive. For the vision/spec, see `scroll_core/docs/scrolls` for lore and specs. For ongoing alignment work, see `docs/alignment/` at the repo root.

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


## Migration Notes (Ledger Service)

- New `invocation::ledger_service` provides a single long‑lived async worker with a bounded `mpsc` channel for ledger writes. Hot paths call `LedgerHandle::try_log(...)` which is non‑blocking and will apply backpressure by returning `TrySendError::Full` when saturated.
- The worker buffers events in an in‑memory staging queue while the DB is not initialized. It drops the oldest item on overflow. When the DB becomes ready, it flushes the buffer and proceeds with normal writes.
- The CLI initializes the DB and runs migrations before starting the service. On shutdown, it drops the handle and awaits the worker with a short timeout for a clean exit.

### Environment

- `DATABASE_URL` (optional): Defaults to `sqlite://scroll_core.db?mode=rwc`. The CLI will create a connection and run migrations on startup.
- SeaORM-only: The codebase uses SeaORM exclusively for sessions and the invocation ledger. Legacy SQLx paths have been removed to simplify builds and avoid OpenSSL toolchains.

### Verifying Locally

- Run the application as usual; under burst load you should see lower CPU usage compared to per‑call thread/runtime spawns.
- Unit tests include coverage for buffering, backpressure, and happy‑path writes. If building offline, you may need to ensure dependencies are pre‑fetched.

### Rollback Steps

- Revert to the previous per‑call ledger write path by removing the `with_ledger(...)` injection on `InvocationManager` and restoring the former thread/runtime spawn blocks.


