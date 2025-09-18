# Alignment Overview

Use this index to understand where product plans, roadmaps, and progress live. Treat it as the first stop before diving into individual documents.

## Current Sprint
- `HANDOFF.md`: sprint snapshot, protocol reminder, verification checklist, and immediate next actions. Read this first when picking up the work.
- `../AGENT_PROTOCOL.md`: full agent workflow (orientation, context categories, logging). Follow it each session and update it when processes shift.
- Follow the protocol in `HANDOFF.md` to log your session in `docs/dev_logs/` and update this README when the sprint focus changes.

## Strategic Roadmap
- `PLAN.md`: phase roadmap and high-level priorities (updated each cycle).
- `MASTER_PLAN.md`: long-range north star and multi-phase sequencing.
- `FEATURES.md`: catalogue of major feature bets with status tags.

## Current Execution
- `CHECKLIST.md`: active sprint / iteration checklist (what's in flight now).
- `PRD.md`, `AgentEnablement_PRD.md`: detailed specs for current initiatives (link back to the roadmap items they deliver).

## Decisions & Evidence
- `DECISIONS.md`: ADR-style log of decisions and rationale.
- `PROGRESS.md`: chronological log of updates, wins, and notable experiments.
- `CLIPPY_BACKLOG.md`: lint/technical-debt backlog captured from tooling (clear before re-enabling full CI).

## Rituals
- Update the roadmap (`PLAN.md`, `FEATURES.md`) at least monthly or when goals shift.
- Refresh `CHECKLIST.md` and related PRDs at the start/end of each sprint.
- Record major architecture/process choices in `DECISIONS.md` immediately.
- Append to `PROGRESS.md` during weekly reviews so history stays current.

Link to this README from code, scrolls, or PRs when pointing contributors to planning artifacts.