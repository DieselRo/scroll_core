---
title: Constructs
status: active
audience: dev
---

# Constructs

Constructs are named agent implementations registered in the runtime.

- Registry: `scroll_core/src/constructs/mod.rs`, `core/construct_registry.rs`
- Metadata: `constructs/construct_metadata.rs`
- Built-ins: `invocation/constructs/*`

Key flows
- Register: on startup, runtime inserts built-in constructs (or mocks)
- Invoke: `InvocationManager` calls selected construct with context frame
- Return: produce text and optional artifacts

Testing
- CLI supports `SCROLL_CORE_USE_MOCK=1` to swap in deterministic outputs


