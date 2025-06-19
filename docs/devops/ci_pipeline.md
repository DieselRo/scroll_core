# CI Pipeline

Branches prefixed with `feat/`, `fix/`, or `chore/` trigger the automated CI and merge flow. Each push or pull request runs formatting, linting and tests via `cargo fmt -- --check`, `cargo clippy -- -D warnings` and `cargo test --all`.

To skip CI for a commit or PR, include `[ci skip]` in the commit message or pull request title/body.

After at least one approving review and all checks succeed, the `auto-merge` workflow automatically squashes and merges the branch into `main` and deletes it.

When `main` is updated, the `tag-release` workflow reads the version from `scroll_core/Cargo.toml` and creates a corresponding `vX.Y.Z` tag if it does not already exist.

Manual overrides can be performed by disabling auto merge on the pull request or manually creating/pushing tags.
