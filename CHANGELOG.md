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
