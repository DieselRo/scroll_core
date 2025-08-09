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

## Regenerating Documentation

Run `cargo xtask gen-map` to regenerate `docs/module_map.md` after code changes.

## Alignment Docs

Active plan, progress, and specs for aligning the runtime with the scrollbooks are tracked in `docs/alignment/`.

## Ledger & Database

The runtime logs invocations to SQLite. Set `DATABASE_URL` to point at a writable
location (default `sqlite://scroll_core.db?mode=rwc`). During local development
you can run `cargo test ledger_service_tests -- --nocapture` to verify the
ledger service buffers and flushes as expected.
