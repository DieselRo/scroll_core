# Changelog

## [0.2.0] - 2025-04-23
### Added
- Extended `Scroll` struct with canonical fields `tags`, `archetype` and `quorum_required`.
- Added `MythicValidated` and `Deprecated` variants to `ScrollStatus`.
- Bumped library version constants to `0.2.0`.

## [0.2.1] - 2025-06-19
### Removed
- Removed episodic_writer module (superseded by stream_writer).

## [0.2.2] - Unreleased
### Changed
- Renamed `context_frame_engine` module to `context_manager`.
- Renamed `invocation_core` to `invocation` and `runner_core` to `runner`.
- Added `ScrollBuilder` and refactored public API.
- 📝 Documentation: expanded Construct directory; added 25 module doc-comments.
### Added
- ModelRegistry: central provider/model resolution with ENV > YAML > defaults
- CLI flag `--print-model-config` to dump resolved config (secrets redacted)
- Example config at `config/models.example.yaml`
- Docs for model config at `docs/reference/models-config.md`
 - Semantic index cache with persistence + incremental rebuild
 - CLI flags `--rebuild-index`, `--reindex <path>` for cache control
 - ENV: `SC_DISABLE_INDEX_CACHE`, `SC_INDEX_CACHE_DIR`, `SC_REBUILD_INDEX`, `SC_REINDEX_PATH`, `SC_EMBEDDER_MODEL`, `SC_EMBEDDING_DIM`
