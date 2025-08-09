# Scroll Core

This repository contains the Scroll Core project.

## LLM Client Configuration

- Provider: `SC_LLM_PROVIDER` = `openai` (default) or `mock`
- OpenAI auth: set `OPENAI_API_KEY` for the `openai` provider
- Timeouts/retries (optional):
  - `SC_LLM_GLOBAL_TIMEOUT_MS` (default 15000)
  - `SC_LLM_ATTEMPT_TIMEOUT_MS` (default 10000)
  - `SC_LLM_MAX_RETRIES` (default 2)
  - `SC_LLM_BASE_BACKOFF_MS` (default 200)
- Model/endpoint overrides:
  - `SC_LLM_MODEL` (default `gpt-4o`)
  - `SC_LLM_ENDPOINT` (default OpenAI chat completions)

In tests or offline, set `SC_LLM_PROVIDER=mock` to avoid network calls.

## Database / Ledger Service

- `DATABASE_URL` (optional): Defaults to `sqlite://scroll_core.db?mode=rwc`.
- On CLI startup, migrations run and the new ledger service starts a single async worker.
- Hot paths log via a non-blocking handle; under load, the bounded channel applies backpressure and may drop new events (caller gets `TrySendError::Full`).

### Local Test Instructions (offline-friendly)

- Many tests require only in-memory SQLite and do not need network. If `cargo test` attempts to fetch new crates, pre-fetch dependencies or run with network enabled.
- To smoke test ledger buffering locally:
  - Set `DATABASE_URL=sqlite::memory:` and run `cargo test ledger_service_tests -- --nocapture`.

## Regenerating Documentation

Run `cargo xtask gen-map` to regenerate `docs/module_map.md` after code changes.

## Alignment Docs

Active plan, progress, and specs for aligning the runtime with the scrollbooks are tracked in `docs/alignment/`.

## Ledger Service

Invocation logs are written by a background ledger service. Events are
sent over a bounded channel and flushed to the database asynchronously.
Set `DATABASE_URL` to change the SQLite location used during local
testing.
