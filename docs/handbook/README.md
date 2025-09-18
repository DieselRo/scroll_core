---
title: Scroll Core Handbook
status: active
audience: dev
---

# Scroll Core Handbook

## Documentation Maintenance

Run the `tools/doc_maintenance` helper to regenerate the documentation indexes in one step.

```sh
./tools/doc_maintenance
```

The command runs the following actions sequentially and reports a summary at the end:

1. `cargo run -- doc --action index`
2. `cargo run -- doc --action normalize`
3. `cargo run -- doc --action classify`

If any action fails, the helper exits with a non-zero code so CI or local scripts can detect the failure quickly. Use the standard cargo output from each step to diagnose issues.
