---
title: Agent Protocol
status: active
audience: agent
---

# Global Agent Protocol

Use this protocol every time a new agent (human or automated) picks up work on Scroll Core. It compresses the shared
context and enforces a consistent flow so we do not lose momentum when context windows reset.

## Orientation Sequence
1. Open `docs/README.md` to understand the documentation portal and current navigation.
2. Follow the "Current Sprint" link in `docs/alignment/README.md`, then read `docs/alignment/HANDOFF.md` to load the sprint
   snapshot, risk callouts, and verification checklist.
3. Review `docs/alignment/CHECKLIST.md`, `PLAN.md`, and any relevant PRDs (`PRD.md`, `AgentEnablement_PRD.md`, etc.) to
   confirm the task you are about to continue.
4. Inspect the latest session log in `docs/dev_logs/`. If none exists for the current date, create a new file named
   `YYYY-MM-DD_session.md` before you begin working.
5. Skim the code areas you expect to touch using `docs/reference/doc-index.md`, `docs/module_map.md`, and targeted `rg`
   searches. Pull only what you need into your context window.

Document any missing links or stale data in `docs/alignment/HANDOFF.md` before you leave so the next agent starts with
accurate instructions.

## Required Context Categories
Capture and maintain the following context so the repository stays self-contained:

| Category             | Purpose                                                       | Primary Sources                                                   |
|----------------------|---------------------------------------------------------------|-------------------------------------------------------------------|
| Task descriptions    | Sprint goals, current tasks, outstanding decisions            | `docs/alignment/HANDOFF.md`, `CHECKLIST.md`, `PLAN.md`, PRDs       |
| Tools                | Available automation/scripts, invocation rituals              | `docs/README.md`, `docs/handbook/README.md`, `docs/agent_system.md`|
| Developer persona    | Environment setup, roles, naming conventions                  | `docs/dev_setup.md`, `scroll_core/README.md`, `docs/README.md`     |
| Code context         | Module responsibilities, key constructs and dependencies      | `docs/module_map.md`, `scroll_core/src/lib.rs`, per-module docs    |
| Semantic structure   | Architecture principles, design constraints, lore             | `docs/concepts/archive.md`, `docs/architecture/*`                  |
| Historical context   | Previous decisions, progress updates, session logs            | `docs/alignment/DECISIONS.md`, `PROGRESS.md`, `docs/dev_logs/`     |
| Collaborative norms  | Coding standards, review rules, communication expectations    | `docs/alignment/HANDOFF.md` (Protocol Reminder), `docs/handbook/README.md`, `docs/agent_system.md` |

When you update one category, link the change in the other docs where it is referenced (e.g., add PRD links to the
handoff and mention new tools in the handbook).

## Standard Work Cycle
1. **Plan** - Write a short plan in your session log (`docs/dev_logs/YYYY-MM-DD_session.md`) before running commands or
   editing code. Record assumptions and the files you expect to touch.
2. **Execute** - Make the smallest viable change, citing documentation as you go. Keep context windows focused; prefer
   summaries and targeted searches (`rg`) over loading entire files.
3. **Validate** - Run the commands from the verification checklist (below) that apply to your change. Document results in
   your session log, even if everything passes.
4. **Summarize** - Update `docs/alignment/HANDOFF.md` and the session log with:
   - Tasks completed and remaining work
   - Any errors encountered and where to reproduce them
   - Updated links to PRs, branches, or review artifacts

## Verification Checklist
Run the full suite when you modify Rust code, rituals, or documentation that affects tooling. For doc-only updates, run
what is relevant.

- `cargo fmt -- --check`
- `cargo test --workspace -- --nocapture`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo +nightly udeps --workspace --all-targets`
- `cargo deny check`
- `cargo xtask gen-map --check`
- `cargo run -- doc --action index`
- `cargo deadlinks --dir docs --dir scroll_core/src`
- `cargo build --workspace --release`

If a command fails, log the failure (with output summary) in your session log, update `docs/alignment/HANDOFF.md` under
"Open Questions and Risks," and create or link a tracking task if follow-up is required.

## Handoff Requirements
- Ensure `docs/alignment/HANDOFF.md` reflects the latest branch status, next actions, risks, and verification outcomes.
- Append a concise end-of-session summary to the current `docs/dev_logs/` entry (work done, blockers, next owner).
- Mention newly created or removed documents in `docs/reference/doc-index.md` so the search tooling stays accurate.
- Leave TODO comments sparingly; prefer backlog items in `docs/alignment/CHECKLIST.md` or PRDs so they are discoverable.

## Drift Prevention
- Cross-reference the protocol whenever you add new onboarding material or tools; note the change in `docs/dev_logs/`.
- During reviews, confirm the contributor used the protocol by checking for a matching session log and handoff update.
- Revisit this file at the end of each sprint to adjust steps that no longer fit the workflow.