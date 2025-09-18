---
title: Trigger Loom Demo Walkthrough
status: experimental
audience: user, operator
---

# Trigger Loom Demo Walkthrough

This guide wires together the **PulseEcho**, **PulseLogger**, and **MythscribeGate** constructs with the Trigger Loom.

## Run the demo profile

```bash
cargo run -- --trigger-loop --profile demo --ticks 10
```

The demo profile scans your `scrolls/` directory. Any scroll tagged `pulse`
automatically enables `PulseEcho` (interval defaults to one tick). Each cycle
the orchestrator emits a "tick" message that `PulseLogger` records in the
invocation ledger. When not running in CI, a second message is sent to
`MythscribeGate` to showcase an additional ambient activation path.

## CI profile

```bash
cargo run -- --trigger-loop --profile ci --ticks 3
```

CI mode enforces deterministic behaviour: the engine uses a fixed seed, runs a
bounded number of ticks (default 3, override with `--ticks`), and
`MythscribeGate` logs quietly without extra work.

## Inspecting the ledger

After a run the ledger table will contain rows for each activation:

```bash
sqlite3 scroll_core.db 'select invoked, phrase from invocation_ledger;'
```

Example rows:

```
pulse_echo|tick
pulse_logger|tick
```

Each `pulse_echo` tick produces a matching `pulse_logger` entry.

## Example scroll tag

```yaml
---
title: Demo Pulse Trigger
scroll_type: System
tags: [pulse]
emotion_signature: { tone: calm, resonance: soft }
---
```

Place the scroll alongside the canonical archive in `scroll_core/docs/scrolls` (or your custom archive directory) to enable the pulse.

