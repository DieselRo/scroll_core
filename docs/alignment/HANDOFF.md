# Agent Handoff - Sprint: Foundations Tightening

_Last updated: 2025-09-18 (update this stamp when you touch the file)_

## Sprint Snapshot
- Docs consolidation and scroll relocation are in progress; runtime and CLI guides now point at the docs portal but the scroll relocation work still needs validation.
- Slim CI is merged (fmt + tests); linting, deadlinks, and release builds are still manual.
- Archive defaults are being re-evaluated; do not assume the old `scrolls/` layout until the relocation checklist closes.

## Branch and PR Status
- Working branch: `master` @ `fc51159`; local tree is dirty with doc moves and scroll archive cleanup.
- No open PRs were left in-flight; push to `master` behind feature toggles while the archive relocation stabilizes.
- Clippy backlog is outstanding; see `CLIPPY_BACKLOG.md` before expanding coverage or tightening gates.

## Immediate Next Actions
1. Implement the archive lint CLI to validate scroll metadata and index alignment.
   - Track requirements in `PLAN.md` (Immediate Front-Loaded Work) and reference runtime notes in `docs/handbook/ci_pipeline.md`.
2. Execute Clippy cleanup wave 1 (context engine, semantic index, model registry, CLI enums) so we can re-enable the full lint gate.
   - Source list lives in `CLIPPY_BACKLOG.md`.
3. Finish wiring docs portal references (README, archive concepts) to the alignment index once the scroll relocation is verified.
   - Checklist updates belong in `CHECKLIST.md` and `PROGRESS.md`.

## Open Questions and Risks
- Scroll relocation: confirm every consumer points at `docs/scrolls` before deleting legacy paths.
- Tooling references: scripts and docs that read from `scrolls/` might break; audit before renaming anything outside the repo root.
- Archive lint CLI design: clarify desired flags/output; align with existing ritual commands to avoid duplicate code paths.

## Verification Checklist
- `cargo fmt -- --check`
- `cargo test --workspace -- --nocapture`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo +nightly udeps --workspace --all-targets`
- `cargo deny check`
- `cargo xtask gen-map --check`
- `cargo run -- doc --action index`
- `cargo deadlinks --dir docs --dir scroll_core/src`
- `cargo build --workspace --release`

If any command fails, record the failure in `PROGRESS.md` and leave a note in the next log entry (see below).

## Protocol Reminder
Follow the detailed workflow in `docs/AGENT_PROTOCOL.md`; use the quick checklist below for orientation.
1. Start at `docs/README.md` for the docs portal; then read `alignment/README.md` (see updated Agent Handoff section).
2. Load task context from `CHECKLIST.md`, `PLAN.md`, and the latest entry in `docs/dev_logs/` (create one if missing for your session).
3. Summarize your plan before coding, keep notes in a dated log under `docs/dev_logs/`, and update this handoff before you exit.

## Logs and References
- Progress log: `PROGRESS.md` (append noteworthy wins or blockers).
- Session logs: drop new files in `docs/dev_logs/` using `YYYY-MM-DD_session.md` naming.
- CI playbook and log retrieval instructions: `docs/devops/ci_pipeline.md` and handbook references under `docs/handbook/`.
- Clippy backlog details: `CLIPPY_BACKLOG.md` (update after each wave).