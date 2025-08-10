---
title: Threads How-To
status: active
audience: user, operator
---

# Threads How-To

Common recipes for working with open threads via the CLI.

## Create a thread

```
cargo run -- open-threads --action create --title "Demo" --scroll scrolls/a.md --tags docs,bug --due-at 2025-12-24T00:00:00Z
```

## List with filters

```
cargo run -- open-threads --action list --mine --overdue --filter-priority HIGH --filter-tags bug,docs --sort updated --limit 10
```

## Close or reopen

```
cargo run -- open-threads --action close <id> --reason "done"
cargo run -- open-threads --action reopen <id> --reason "revisit"
```

## Nudge overdue

```
cargo run -- open-threads --action nudge
```

### Windows hints

Use an absolute SQLite URL such as `sqlite:///C:/dev/scroll_core/scroll_core.db` and `--mine` falls back to the `USERNAME` environment variable when `USER` is not set.
