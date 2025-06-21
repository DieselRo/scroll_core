# Development Setup

Quick start commands:

```bash
# VS Code
gh repo clone DieselRo/scroll_core
code scroll_core # VS Code prompts to "Reopen in Container"

# Or Nix
nix develop
```

Some CI checks use nightly-only flags. To run them locally, install the nightly
toolchain and invoke commands with `cargo +nightly`, for example:

```bash
rustup toolchain install nightly
cargo +nightly udeps --workspace --all-targets
```

