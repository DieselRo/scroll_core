# Semantic Index Errors

`build_semantic_index` now returns `Result<(), ArchiveError>`. The main error variants are:

- `EmptyScrollSet` – no scrolls were provided.
- `MissingModel` – an embedding model path was expected but not found.
- `EmbeddingFailure` – embedding a scroll failed with a message.

The function accepts an `Embedder` trait object. Production code can supply a real model-based embedder. Tests may use `MockEmbedder` or the default `TokenEmbedder` to generate deterministic vectors.
