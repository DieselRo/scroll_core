---
title: Trigger Loom Demo Walkthrough
status: experimental
audience: user, operator
---

# Trigger Loom Demo Walkthrough

This guide wires together the **PulseEcho**, **PulseLogger**, and **MythscribeGate** constructs with the Trigger Loom.

## Run the demo profile

```bash
cargo run -- --trigger-loop --profile demo
```

The demo profile scans your `scrolls/` directory. Any scroll tagged `pulse` awakens `PulseEcho`, which emits a bus message each tick. `PulseLogger` listens on the bus and records the activity in the invocation ledger.

## CI profile

```bash
cargo run -- --trigger-loop --profile ci
```

CI mode runs a small, deterministic number of ticks and short‑circuits heavy work in `MythscribeGate`.

## Inspecting the ledger

After a run the ledger table will contain rows for each activation:

```bash
sqlite3 scroll_core.db 'select invoked, phrase from invocation_ledger;'
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

Place the scroll in your `scrolls/` directory to enable the pulse.
