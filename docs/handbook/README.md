---
title: Scroll Core Handbook
status: active
audience: dev
---

# Scroll Core Handbook

## Documentation Maintenance

Run the `tools/doc_maintenance` helper to regenerate the documentation indexes in one step.

```sh
./tools/doc_maintenance
```

The command runs the following actions sequentially and reports a summary at the end:

1. `cargo run -- doc --action index`
2. `cargo run -- doc --action normalize`
3. `cargo run -- doc --action classify`

If any action fails, the helper exits with a non-zero code so CI or local scripts can detect the failure quickly. Use the standard cargo output from each step to diagnose issues.
## CI Scope (Temporary)

Our GitHub workflows currently run only `cargo fmt -- --check` and `cargo test --workspace -- --nocapture`. Run the full suite (`cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo +nightly udeps --workspace --all-targets`, `cargo deny check`, `cargo xtask gen-map --check`, `cargo deadlinks`, `cargo test`, `cargo build --release`) locally before pushing when you touch areas listed in `docs/alignment/CLIPPY_BACKLOG.md`.

## CI Log Retrieval

Follow these steps after a pull request or branch push to review CI output:

1. Open the PR and visit the `Checks` tab; select the failing or completed workflow run.
2. Within the job summary, locate the `Artifacts` panel and download the `ci-log-ubuntu` or `ci-log-windows` bundle.
3. Alternatively, use the GitHub CLI (requires `gh auth login`):
   ```powershell
   gh run download --repo <owner>/<repo> --name ci-log-ubuntu --dir CICDLogs
   ```
   Replace `ci-log-ubuntu` with `ci-log-windows` for the Windows job or omit `--name` to list available artifacts.
4. Extract the archive; the combined stdout/stderr lives in `ci.log`. Share it in the PR discussion or drop it into a scratch directory for analysis.
5. When handing the log to agents, mention the artifact name and location so they can reference it in final reports (see `docs/agent_system.md#appendix-a-agent-playbook`).
