# Scroll Core – Implementation Notes

This crate implements the runtime for The Archive. For the vision/spec, see the `scrolls/` directory. For ongoing alignment work, see `docs/alignment/` at the repo root.

Key entry points:
- `src/main.rs`: CLI entry
- `archive/`: loading, in-memory archive, semantic index
- `core/`: context framing, cost manager
- `invocation/`: constructs and invocation manager
- `chat/`: interactive chat and routing

## Migration Notes (Ledger Service)

Invocation logging now uses a long-lived asynchronous service. It accepts
events over a bounded channel and persists them once the database is ready.

- Database URL is read from `DATABASE_URL` and defaults to
  `sqlite://scroll_core.db?mode=rwc`.
- To disable the service, set `SC_LEDGER_DISABLE=1`.
- The service can be started with `ledger_service::start` and shut down with
  `LedgerService::shutdown`.


