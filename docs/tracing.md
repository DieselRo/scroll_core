# Tracing

Scroll Core emits structured logs via the `tracing` crate. The default format is JSON.

Enable debug logs:

```bash
RUST_LOG=scroll_core=debug cargo run -- chat mythscribe --no-stream
```

Pipe logs into tools like `jq`:

```bash
RUST_LOG=scroll_core=info cargo run | jq .
```

Set `SCROLL_TRACE_FORMAT=pretty` or compile with the `compact_tracing` feature to use human friendly output.
