
### A5. Manual Lint Suite

Run the full local workflow before pushing substantial changes:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo +nightly udeps --workspace --all-targets
cargo deny check
cargo xtask gen-map --check
cargo run -- doc --action normalize
cargo run -- doc --action classify
cargo deadlinks --dir docs --dir scroll_core/src
cargo test --workspace -- --nocapture
cargo build --workspace --release
```

Record any failures in the final report and reference the specific command that triggered them.
