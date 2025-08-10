---
title: Auditing Context Decisions
status: draft
audience: dev
---

# Auditing Context Decisions

The context ledger records how each frame was assembled. Use the CLI to
inspect recent frames or export them for analysis.

## Examples

```bash
cargo run -- context --limit 10
cargo run -- context --limit 5 --details
cargo run -- context --limit 5 --details --export yaml
```

`--details` includes candidate rows. `--export` emits JSON or YAML instead
of the human-readable table.

Set `SC_CONTEXT_DECISIONS_VERBOSE=true` or `context.decisions_verbose: true`
in `models.yaml` to persist candidate rows automatically.
