---
title: Context Frame Engine
status: active
audience: dev
---

# Context Frame Engine

Builds a layered context for each invocation:

- Inputs: session transcript, archive excerpts, construct metadata
- Process: retrieve → filter → compose frame → trim tokens
- Output: prompt-ready structured context

Code
- `scroll_core/src/core/context_frame_engine.rs`
- Retrieval utilities under `archive/` and `memory/`


