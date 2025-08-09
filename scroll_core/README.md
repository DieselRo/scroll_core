# Scroll Core – Implementation Notes

This crate implements the runtime for The Archive. For the vision/spec, see the `scrolls/` directory. For ongoing alignment work, see `docs/alignment/` at the repo root.

Key entry points:
- `src/main.rs`: CLI entry
- `archive/`: loading, in-memory archive, semantic index
- `core/`: context framing, cost manager
- `invocation/`: constructs and invocation manager
- `chat/`: interactive chat and routing

## Migration Notes (Ledger Service)

Invocations are now logged via a single async ledger service. The service accepts
events over a bounded channel and persists them once the database is ready.

### Environment
- `DATABASE_URL` (defaults to `sqlite://scroll_core.db?mode=rwc`)

### Verify
1. Run migrations and start the CLI: `cargo run`
2. Observe `invocation_ledger` rows increasing after invocations.

### Rollback
Stop starting the ledger service in `main.rs` and logging becomes a no-op.


