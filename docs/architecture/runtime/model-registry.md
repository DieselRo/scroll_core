---
title: Model Registry
status: active
audience: dev
---

# Model Registry

Purpose
- Central place to register and resolve models used by constructs.

Code
- `scroll_core/src/models/mod.rs`
- `scroll_core/src/models/base_model.rs`
- `scroll_core/src/models/model_registry.rs`

Notes
- Constructs select a model via the registry; OpenAI-backed constructs require `OPENAI_API_KEY`.
- Future: per-construct and per-session model policies.


