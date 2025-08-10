---
title: CLI Command Reference
status: active
audience: user, dev
---

# CLI Command Reference

## Chat

```
cargo run -- chat <construct> [--stream|--no-stream] [--no-banner]
```

- Talk to Mythscribe: `cargo run -- chat mythscribe --no-stream`

## Rituals

- Validate one: `cargo run -- ritual --action validate --file <relpath>`
- Validate all: `cargo run -- ritual --action validate-all`
- Write (persist) and index: `cargo run -- ritual --action write --file <relpath> --update-index`
- Seal: `cargo run -- ritual --action seal --file <relpath>`

## Index

- List: `cargo run -- index --action list`
- Add: `cargo run -- index --action add --file <relpath>`
- Remove: `cargo run -- index --action remove --file <relpath>`

## Docs maintenance

- Build index: `cargo run -- doc --action index`
- Recent report: `cargo run -- doc --action recent`
- Classify (json/md): `cargo run -- doc --action classify`
- Normalize headers: `cargo run -- doc --action normalize`
- Generate master plan: `cargo run -- doc --action master-plan`
- Fix missing headers (adds minimal YAML): `cargo run -- doc --action classify --fix-headers`

## Open Threads

- Create: `cargo run -- open-threads --action create --title "<title>" --scroll <path> [--assignee <name>] [--priority HIGH|MEDIUM|LOW] [--tags a,b] [--due-at 2025-08-11T12:00:00Z]`
- List: `cargo run -- open-threads --action list [--status OPEN] [--scroll <path>] [--limit N] [--mine] [--overdue] [--filter-priority HIGH] [--filter-tags a,b] [--sort created|updated|priority]`
- Close: `cargo run -- open-threads --action close <id> [--reason <text>]`
- Reopen: `cargo run -- open-threads --action reopen <id> [--reason <text>]`
- Nudge: `cargo run -- open-threads --action nudge`


