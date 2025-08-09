---
title: Artifacts
status: active
audience: dev
---

# Artifacts

Artifacts are durable outputs produced by constructs (e.g., generated files).

Code
- Trait: `scroll_core/src/artifact.rs` (`WritableArtifact`)
- Services: `scroll_core/src/artifacts/artifact_service.rs`
- Module: `scroll_core/src/artifacts/mod.rs`

Lifecycle
- Construct emits an artifact → service persists → index references

Future
- Content addressing, metadata, and retention policies


