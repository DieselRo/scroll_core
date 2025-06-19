# Vector Memory Metrics

The vector memory system exposes Prometheus metrics when Scroll Core is built with the `metrics` feature. Metrics are served on port `9898` by the built‑in exporter.

### Histograms
- `scroll_embed_time_seconds` – time spent generating a vector embedding for a single scroll.
- `vector_index_update_time_seconds` – total time taken to rebuild the semantic index.
- `vector_index_memory_bytes` – memory consumed by the index after an update.

Run `curl localhost:9898/metrics` to view raw metrics or import `docs/dashboards/vector_memory.json` into Grafana for visualization.
