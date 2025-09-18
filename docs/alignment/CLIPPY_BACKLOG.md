# Clippy Remediation Backlog

The current `cargo clippy --workspace --all-targets --all-features -- -D warnings` run surfaces the issues below. Address
items under each heading before we re-enable the full lint gate.

## Runtime
- `core/context_frame_engine.rs`: simplify the semantic candidate pipeline (remove manual `filter_map`, redundant field
  names, casts) and keep metrics calls on the builder APIs.
- `archive/semantic_index.rs`: prefer `std::io::Error::other` without wrapping closures.
- `models/model_registry.rs`: use `or_default` in place of manual `or_insert_with` calls.
- `threads/*` services: either refactor function signatures to reduce parameters or document with targeted
  `#[allow(clippy::too_many_arguments)]` after evaluation.

## CLI & Runtime Tests
- `tests/trigger_loom_v1_tests.rs`: remove unused assignments when running async sections.
- `tests/semantic_index_cache.rs`: drop needless lifetimes and replace manual `if let` flattening with iterator helpers.
- `tests/autocapture_validator.rs`: prune unused imports and duplicate `#[ignore]` annotations (Windows gating already in
  place).

## CLI enum ergonomics
- `src/main.rs`: the `Commands` enum is flagged as a large variant; consider boxing the heavy fields or splitting
  subcommands.
- Several command handlers construct services with `&conn` where the method expects ownership; remove needless borrows.

## Follow-up
- After the above are cleared, re-run the full workflow locally (fmt, clippy, udeps, deny, doc tooling, deadlinks, test,
  release build) before re-introducing the stricter CI jobs.
